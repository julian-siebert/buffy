use crate::{
    command::CommandWrapper,
    compiler::Compiler,
    compilers::collect_proto_files,
    config::Config,
    error::{Error, IoResultExt, Result},
};
use async_trait::async_trait;
use console::style;
use indicatif::ProgressBar;
use std::path::PathBuf;
use tempfile::TempDir;
use tera::{Context, Tera};
use tokio::{process::Command, sync::Mutex};
use which::which;

const CARGO_TOML_TEMPLATE: &str = include_str!("templates/Cargo.toml.tera");

pub struct RustCompiler {
    protoc: PathBuf,
    cargo: PathBuf,
    work_dir: Mutex<Option<TempDir>>,
}

impl RustCompiler {
    pub fn new(grpc: bool) -> Result<Self> {
        let protoc = which("protoc").map_err(|_| Error::MissingProgram {
            program: "protoc".into(),
        })?;

        which("protoc-gen-prost").map_err(|_| Error::MissingProgram {
            program: "protoc-gen-prost".into(),
        })?;
        which("protoc-gen-prost-crate").map_err(|_| Error::MissingProgram {
            program: "protoc-gen-prost-crate".into(),
        })?;

        if grpc {
            which("protoc-gen-tonic").map_err(|_| Error::MissingProgram {
                program: "protoc-gen-tonic".into(),
            })?;
        }

        let cargo = which("cargo").map_err(|_| Error::MissingProgram {
            program: "cargo".into(),
        })?;

        Ok(Self {
            protoc,
            cargo,
            work_dir: Mutex::new(None),
        })
    }

    async fn get_work_dir(&self) -> tokio::sync::MutexGuard<'_, Option<TempDir>> {
        self.work_dir.lock().await
    }

    async fn resolve_tonic_version(configured: Option<&String>) -> Result<String> {
        if let Some(v) = configured {
            return Ok(v.clone());
        }

        let client = reqwest::Client::builder()
            .user_agent("buffy-build-tool/1.0")
            .build()
            .map_err(|e| Error::Internal(format!("HTTP client error: {e}")))?;

        let body = client
            .get("https://crates.io/api/v1/crates/tonic")
            .send()
            .await
            .map_err(|e| Error::Internal(format!("crates.io API unreachable: {e}")))?
            .text()
            .await
            .map_err(|e| Error::Internal(format!("crates.io API read error: {e}")))?;

        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            Error::Internal(format!("crates.io API parse error: {e}\nResponse: {body}"))
        })?;

        json["crate"]["newest_version"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Internal(
                    "Could not resolve tonic version from crates.io.\n\
                 Pin it manually in Buffy.toml:\n\
                 [rust]\n\
                 tonic_version = \"0.12.3\""
                        .into(),
                )
            })
    }

    async fn resolve_prost_version(configured: Option<&String>) -> Result<String> {
        if let Some(v) = configured {
            return Ok(v.clone());
        }

        let client = reqwest::Client::builder()
            .user_agent("buffy-build-tool/1.0")
            .build()
            .map_err(|e| Error::Internal(format!("HTTP client error: {e}")))?;

        let body = client
            .get("https://crates.io/api/v1/crates/prost")
            .send()
            .await
            .map_err(|e| Error::Internal(format!("crates.io API unreachable: {e}")))?
            .text()
            .await
            .map_err(|e| Error::Internal(format!("crates.io API read error: {e}")))?;

        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            Error::Internal(format!("crates.io API parse error: {e}\nResponse: {body}"))
        })?;

        json["crate"]["newest_version"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| {
                Error::Internal(
                    "Could not resolve prost version from crates.io.\n\
                 Pin it manually in Buffy.toml:\n\
                 [rust]\n\
                 prost_version = \"0.13.5\""
                        .into(),
                )
            })
    }

    fn render_cargo_toml(
        cfg: &Config,
        rust_cfg: &crate::config::Rust,
        version: &str,
        prost_version: &str,
        tonic_version: Option<&str>,
        grpc: bool,
    ) -> Result<String> {
        let mut tera = Tera::default();
        tera.add_raw_template("Cargo.toml", CARGO_TOML_TEMPLATE)
            .map_err(|e| Error::Internal(format!("Tera template error: {e}")))?;

        let lib_name = rust_cfg.name.replace('-', "_");

        let mut ctx = Context::new();
        ctx.insert("name", &rust_cfg.name);
        ctx.insert("description", &cfg.package.description);
        ctx.insert("lib_name", &lib_name);
        ctx.insert("version", version);
        ctx.insert("edition", &rust_cfg.edition);
        let authors_toml: Vec<String> = cfg
            .package
            .authors
            .iter()
            .map(|a| format!("\"{a}\""))
            .collect();
        ctx.insert("authors", &authors_toml);
        ctx.insert("license", &cfg.package.license);
        ctx.insert("documentation", &rust_cfg.documentation);
        ctx.insert("homepage", &rust_cfg.homepage);
        ctx.insert("repository", &rust_cfg.repository);
        ctx.insert("prost_version", prost_version);
        ctx.insert("grpc", &grpc);
        ctx.insert("tonic_version", &tonic_version.unwrap_or(""));

        tera.render("Cargo.toml", &ctx)
            .map_err(|e| Error::Internal(format!("Tera render error: {e}")))
    }
}

#[async_trait]
impl Compiler for RustCompiler {
    async fn build(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        let rust_cfg = match cfg.rust.as_ref() {
            Some(c) => c,
            None => return Ok(()),
        };

        pb.set_message("Resolving prost version...");
        let prost_version = Self::resolve_prost_version(rust_cfg.prost_version.as_ref()).await?;

        let tonic_version = if cfg.package.grpc {
            let v = Self::resolve_tonic_version(rust_cfg.tonic_version.as_ref()).await?;
            pb.suspend(|| {
                eprintln!(
                    "{} {} using prost {} + tonic {}",
                    style("[i]").cyan().bold(),
                    style("RUST").bold(),
                    style(format!("v{}", &prost_version)).yellow(),
                    style(format!("v{v}")).yellow(),
                );
            });
            Some(v)
        } else {
            pb.suspend(|| {
                eprintln!(
                    "{} {} using prost {}",
                    style("[i]").cyan().bold(),
                    style("RUST").bold(),
                    style(format!("v{}", &prost_version)).yellow(),
                );
            });
            None
        };

        let tmp = tempfile::Builder::new()
            .prefix("buffy-rust-")
            .tempdir()
            .io_err()?;
        let dir = tmp.path().to_path_buf();
        let src_dir = dir.join("src");
        std::fs::create_dir_all(&src_dir).io_err()?;

        pb.set_message("Generating Cargo.toml...");
        let cargo_toml = Self::render_cargo_toml(
            &cfg,
            rust_cfg,
            &cfg.package.version.to_string(),
            &prost_version,
            tonic_version.as_deref(),
            cfg.package.grpc,
        )?;
        std::fs::write(dir.join("Cargo.toml"), cargo_toml).io_err()?;

        pb.set_message("Generating Rust code from proto files...");
        let proto_files = collect_proto_files(&cfg.source.path)?;

        let mut protoc_cmd = Command::new(&self.protoc);
        protoc_cmd
            .arg(format!("--prost_out={}", src_dir.display()))
            .arg(format!("--proto_path={}", cfg.source.path.display()));

        if cfg.package.grpc {
            protoc_cmd.arg(format!("--tonic_out={}", src_dir.display()));
        }

        protoc_cmd.args(&proto_files);

        CommandWrapper::new(&mut protoc_cmd, "rust")
            .progress(pb.clone())
            .run()
            .await?;

        pb.set_message("Generating lib.rs...");

        fn collect_files_recursive(
            dir: &std::path::Path,
        ) -> Result<std::collections::HashMap<PathBuf, String>> {
            let mut files = std::collections::HashMap::new();
            for entry in std::fs::read_dir(dir).io_err()? {
                let entry = entry.io_err()?;
                let path = entry.path();
                if path.is_dir() {
                    files.extend(collect_files_recursive(&path)?);
                } else {
                    let content = std::fs::read_to_string(&path).io_err()?;
                    files.insert(path.to_path_buf(), content);
                }
            }
            Ok(files)
        }

        fn build_lib_rs(
            src_dir: &std::path::Path,
            files: &std::collections::HashMap<PathBuf, String>,
        ) -> String {
            let resolve_content = |content: &str, _dir: &std::path::Path| -> String {
                let mut result = content.to_string();
                for (path, other_content) in files {
                    if let Some(name) = path.file_name() {
                        let name = name.to_string_lossy();
                        result = result.replace(&format!("include!(\".{name}\");"), other_content);
                        result = result.replace(&format!("include!(\"{name}\");"), other_content);
                    }
                }
                result
            };

            fn build_module(
                dir: &std::path::Path,
                src_dir: &std::path::Path,
                files: &std::collections::HashMap<PathBuf, String>,
                resolve: &dyn Fn(&str, &std::path::Path) -> String,
            ) -> String {
                let mut output = String::new();

                let mut entries: Vec<_> = std::fs::read_dir(dir)
                    .unwrap()
                    .filter_map(|e| e.ok())
                    .collect();
                entries.sort_by_key(|e| e.path());

                for entry in &entries {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path.file_name().unwrap().to_string_lossy();

                        if name == "lib.rs" || name.starts_with('.') || name.ends_with(".tonic.rs")
                        {
                            continue;
                        }

                        if let Some(content) = files.get(&path) {
                            output.push_str(&resolve(content, path.parent().unwrap()));
                            output.push('\n');
                        }
                    }
                }

                for entry in &entries {
                    let path = entry.path();
                    if path.is_dir() {
                        let mod_name = path.file_name().unwrap().to_string_lossy();
                        let inner = build_module(&path, src_dir, files, resolve);
                        if !inner.trim().is_empty() {
                            output.push_str(&format!("pub mod {mod_name} {{\n"));
                            for line in inner.lines() {
                                output.push_str(&format!("    {line}\n"));
                            }
                            output.push_str("}\n\n");
                        }
                    }
                }

                output
            }

            build_module(src_dir, src_dir, files, &resolve_content)
        }

        let files = collect_files_recursive(&src_dir)?;
        let lib_rs = build_lib_rs(&src_dir, &files);

        for entry in std::fs::read_dir(&src_dir).io_err()? {
            let entry = entry.io_err()?;
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path).io_err()?;
            } else if path != src_dir.join("lib.rs") {
                std::fs::remove_file(&path).io_err()?;
            }
        }

        std::fs::write(src_dir.join("lib.rs"), lib_rs).io_err()?;

        pb.set_message("Verifying with cargo build...");
        CommandWrapper::new(
            Command::new(&self.cargo)
                .args(["build", "--quiet"])
                .current_dir(&dir),
            "rust",
        )
        .progress(pb.clone())
        .run()
        .await?;

        *self.get_work_dir().await = Some(tmp);

        pb.finish_and_clear();

        Ok(())
    }

    async fn publish(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        let rust_cfg = match cfg.rust.as_ref() {
            Some(c) => c,
            None => return Ok(()),
        };

        let guard = self.get_work_dir().await;
        let dir = guard
            .as_ref()
            .ok_or_else(|| Error::Internal("publish() called before build()".into()))?
            .path()
            .to_path_buf();

        if std::env::var("CARGO_REGISTRY_TOKEN").is_err() {
            return Err(Error::MissingConfig {
                field: "CARGO_REGISTRY_TOKEN".into(),
                hint: indoc::indoc! {"
                    Set this environment variable before publishing:

                    CARGO_REGISTRY_TOKEN – API token from https://crates.io/me
                                           or your self-hosted registry

                    For self-hosted registries also set the index in ~/.cargo/config.toml:
                    [registries]
                    my-registry = { index = \"https://my-kellnr.example.com/index\" }
                "}
                .into(),
            });
        }

        let version = cfg.package.version.to_string();
        pb.set_message(format!(
            "Publishing {} v{version} to {}...",
            rust_cfg.name, rust_cfg.registry
        ));

        let mut args = vec!["publish".to_string(), "--no-verify".to_string()];

        if rust_cfg.registry != "crates-io" {
            args.push("--registry".to_string());
            args.push(rust_cfg.registry.clone());
        }

        CommandWrapper::new(
            Command::new(&self.cargo).args(&args).current_dir(&dir),
            "rust",
        )
        .progress(pb.clone())
        .run()
        .await?;

        pb.finish_and_clear();

        Ok(())
    }
}
