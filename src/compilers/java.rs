use std::path::PathBuf;

use async_trait::async_trait;
use console::style;
use indicatif::ProgressBar;
use tempfile::TempDir;
use tera::{Context, Tera, Value};
use tokio::{process::Command, sync::Mutex};
use which::which;

use crate::{
    command::CommandWrapper,
    compiler::Compiler,
    compilers::collect_proto_files,
    config::Config,
    error::{Error, IoResultExt, Result},
};

const POM_TEMPLATE: &str = include_str!("templates/pom.xml.tera");

pub struct JavaCompiler {
    protoc: PathBuf,
    mvn: PathBuf,
    gpg: PathBuf,
    work_dir: Mutex<Option<TempDir>>,
}

impl JavaCompiler {
    pub fn new() -> Result<Self> {
        let protoc = which("protoc")?;
        let mvn = which("mvn")?;
        let gpg = which("gpg")?;
        Ok(Self {
            protoc,
            mvn,
            gpg,
            work_dir: Mutex::new(None),
        })
    }

    async fn get_work_dir(&self) -> tokio::sync::MutexGuard<'_, Option<TempDir>> {
        self.work_dir.lock().await
    }

    async fn resolve_protobuf_version(configured: Option<&String>) -> Result<String> {
        if let Some(v) = configured {
            return Ok(v.clone());
        }

        let client = reqwest::Client::builder()
            .user_agent("buffy-build-tool/1.0")
            .build()
            .map_err(|e| Error::Internal(format!("HTTP client error: {e}")))?;

        let body = client
            .get("https://search.maven.org/solrsearch/select?q=g:com.google.protobuf+AND+a:protobuf-java&rows=1&wt=json")
            .send()
            .await
            .map_err(|e| Error::Internal(format!("Maven Central API unreachable: {e}")))?
            .text()
            .await
            .map_err(|e| Error::Internal(format!("Maven Central API read error: {e}")))?;

        let json: Value = serde_json::from_str(&body).map_err(|e| {
            Error::Internal(format!(
                "Maven Central API parse error: {e}\nResponse: {body}"
            ))
        })?;

        json["response"]["docs"][0]["latestVersion"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Internal(
                    "Could not resolve protobuf-java version from Maven Central.\n\
                 Pin it manually in Buffy.toml:\n\
                 [java]\n\
                 protobuf_version = \"4.29.3\""
                        .into(),
                )
            })
    }

    fn render_pom(
        cfg: &Config,
        java: &crate::config::Java,
        protobuf_version: &str,
    ) -> Result<String> {
        let mut tera = Tera::default();
        tera.add_raw_template("pom.xml", POM_TEMPLATE)
            .map_err(|e| Error::Internal(format!("Tera template error: {e}")))?;

        let mut ctx = Context::new();
        ctx.insert("group_id", &java.group_id);
        ctx.insert("artifact_id", &java.artifact_id);
        ctx.insert("version", &cfg.package.version.to_string());
        ctx.insert("description", &java.description);
        ctx.insert("url", &java.url);
        ctx.insert("license_name", &java.license.name);
        ctx.insert("license_url", &java.license.url);
        ctx.insert("developer_id", &java.developer.id);
        ctx.insert("developer_name", &java.developer.name);
        ctx.insert("developer_email", &java.developer.email);
        ctx.insert("scm_connection", &java.scm.connection);
        ctx.insert("scm_url", &java.scm.url);
        ctx.insert("protobuf_version", protobuf_version);

        tera.render("pom.xml", &ctx)
            .map_err(|e| Error::Internal(format!("Tera render error: {e}")))
    }
}

#[async_trait]
impl Compiler for JavaCompiler {
    async fn build(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        if cfg.java.is_none() {
            return Ok(());
        }
        let java_cfg = cfg.java.as_ref().unwrap();

        pb.set_message("Resolving protobuf-java version...");
        let protobuf_version =
            Self::resolve_protobuf_version(java_cfg.protobuf_version.as_ref()).await?;

        pb.suspend(|| {
            eprintln!(
                "{} {} using protobuf-java {}",
                style("[i]").cyan().bold(),
                style("JAVA").bold(),
                style(format!("v{}", &protobuf_version)).yellow(),
            );
        });

        let tmp = tempfile::Builder::new()
            .prefix("buffy-java-")
            .tempdir()
            .io_err()?;
        let dir = tmp.path().to_path_buf();

        let src_dir = dir.join("src/main/java");
        std::fs::create_dir_all(&src_dir).io_err()?;

        pb.set_message("Generating pom.xml...");
        let pom = Self::render_pom(&cfg, java_cfg, &protobuf_version)?;
        std::fs::write(dir.join("pom.xml"), pom).io_err()?;

        pb.set_message("Generating Java code from proto files...");
        let proto_files = collect_proto_files(&cfg.source.path)?;

        CommandWrapper::new(
            Command::new(&self.protoc)
                .arg(format!("--java_out={}", src_dir.display()))
                .arg(format!("--proto_path={}", cfg.source.path.display()))
                .args(&proto_files),
            "java",
        )
        .progress(pb.clone())
        .run()
        .await?;

        pb.set_message("Verifying with mvn compile...");
        CommandWrapper::new(
            Command::new(&self.mvn)
                .args(["compile", "-q"])
                .current_dir(&dir),
            "java",
        )
        .progress(pb.clone())
        .run()
        .await?;

        *self.get_work_dir().await = Some(tmp);
        pb.finish_and_clear();
        Ok(())
    }

    async fn publish(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        if cfg.java.is_none() {
            return Ok(());
        }
        let java_cfg = cfg.java.as_ref().unwrap();

        let guard = self.get_work_dir().await;
        let dir = guard
            .as_ref()
            .ok_or_else(|| Error::Internal("publish() called before build()".into()))?
            .path()
            .to_path_buf();

        if let Ok(key) = std::env::var("GPG_PRIVATE_KEY") {
            pb.set_message("Importing GPG key...");
            let key_file = dir.join(".gpg-key.asc");

            std::fs::write(&key_file, &key).io_err()?;

            CommandWrapper::new(
                Command::new(&self.gpg)
                    .args(["--batch", "--import"])
                    .arg(&key_file),
                "java",
            )
            .progress(pb.clone())
            .run()
            .await?;

            std::fs::remove_file(&key_file).ok();
        }

        let username = std::env::var("MAVEN_USERNAME").ok();
        let password = std::env::var("MAVEN_PASSWORD").ok();
        let gpg_key = std::env::var("GPG_KEY_ID").ok();
        let gpg_pass = std::env::var("GPG_PASSPHRASE").ok();

        if username.is_none() || password.is_none() {
            return Err(Error::MissingConfig {
                field: "MAVEN_USERNAME / MAVEN_PASSWORD".into(),
                hint: indoc::indoc! {"
                        Set these environment variables before publishing:

                        MAVEN_USERNAME   – Maven Central username (portal.central.sonatype.com)
                        MAVEN_PASSWORD   – Maven Central token
                        GPG_KEY_ID       – GPG key ID used for signing
                        GPG_PASSPHRASE   – GPG key passphrase
                        GPG_PRIVATE_KEY  – (CI only) armored private key, base64 encoded
                    "}
                .into(),
            });
        }

        let version = cfg.package.version.to_string();
        pb.set_message(format!(
            "Publishing {}:{} v{version} to Maven Central...",
            java_cfg.group_id, java_cfg.artifact_id
        ));

        let mut mvn_args = vec![
            "deploy".to_string(),
            "-P".to_string(),
            "release".to_string(),
            "--batch-mode".to_string(),
            "--no-transfer-progress".to_string(),
            format!("-Dusername={}", username.unwrap()),
            format!("-Dpassword={}", password.unwrap()),
        ];

        if let Some(key_id) = gpg_key {
            mvn_args.push(format!("-Dgpg.keyname={key_id}"));
        }
        if let Some(passphrase) = gpg_pass {
            mvn_args.push(format!("-Dgpg.passphrase={passphrase}"));
        }

        CommandWrapper::new(
            Command::new(&self.mvn).args(&mvn_args).current_dir(&dir),
            "java",
        )
        .progress(pb.clone())
        .run()
        .await?;

        pb.finish_and_clear();

        Ok(())
    }
}
