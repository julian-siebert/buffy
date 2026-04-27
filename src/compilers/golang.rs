use std::path::PathBuf;

use async_trait::async_trait;
use console::style;
use indicatif::ProgressBar;
use tempfile::TempDir;
use tokio::{process::Command, sync::Mutex};
use which::which;

use crate::{
    command::CommandWrapper,
    compiler::Compiler,
    compilers::collect_proto_files,
    config::Config,
    error::{Error, IoResultExt, Result},
    license::resolve_licenses,
};

pub struct GolangCompiler {
    protoc: PathBuf,
    work_dir: Mutex<Option<TempDir>>,
}

impl GolangCompiler {
    pub fn new(grpc: bool) -> Result<Self> {
        let protoc = which("protoc").map_err(|_| Error::MissingProgram {
            program: "protoc".into(),
        })?;
        let _ = which("protoc-gen-go").map_err(|_| Error::MissingProgram {
            program: "protoc-gen-go".into(),
        })?;
        let _ = which("go").map_err(|_| Error::MissingProgram {
            program: "go".into(),
        })?;
        if grpc {
            which("protoc-gen-go-grpc").map_err(|_| Error::MissingProgram {
                program: "protoc-gen-go-grpc".into(),
            })?;
        }
        Ok(Self {
            protoc,
            work_dir: Mutex::new(None),
        })
    }

    async fn get_work_dir(&self) -> tokio::sync::MutexGuard<'_, Option<TempDir>> {
        self.work_dir.lock().await
    }
}

#[async_trait]
impl Compiler for GolangCompiler {
    async fn build(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        if cfg.golang.is_none() {
            return Ok(());
        }

        let golang_config = cfg.golang.as_ref().ok_or_else(|| Error::MissingConfig {
            field: "golang.module".into(),
            hint: r#"Add the following to your Buffy.toml:

        [golang]
        module = "github.com/your-user/your-repo-go""#
                .into(),
        })?;

        let tmp = tempfile::Builder::new()
            .prefix("buffy-golang-")
            .tempdir()
            .io_err()?;

        let dir = tmp.path().to_path_buf();

        pb.set_message("Generating Golang-code...");

        let proto_files = collect_proto_files(&cfg.source.path)?;

        let mut protoc_cmd = Command::new(&self.protoc);
        protoc_cmd
            .arg(format!("--go_out={}", dir.display()))
            .arg(format!("--go_opt=module={}", &golang_config.module))
            .arg(format!("--proto_path={}", cfg.source.path.display()));

        if cfg.package.grpc {
            protoc_cmd
                .arg(format!("--go-grpc_out={}", dir.display()))
                .arg(format!("--go-grpc_opt=module={}", &golang_config.module));
        }

        protoc_cmd.args(&proto_files);

        CommandWrapper::new(&mut protoc_cmd, "golang")
            .progress(pb.clone())
            .run()
            .await?;

        pb.set_message("Initialize go module...");

        CommandWrapper::new(
            Command::new("go")
                .args(["mod", "init", &golang_config.module])
                .current_dir(&dir),
            "golang",
        )
        .progress(pb.clone())
        .run()
        .await?;

        pb.set_message("Tidying up go module...");

        CommandWrapper::new(
            Command::new("go").args(["mod", "tidy"]).current_dir(&dir),
            "golang",
        )
        .progress(pb.clone())
        .run()
        .await?;

        pb.set_message("Writing LICENSE file(s)...");
        let licenses = resolve_licenses(&cfg.package.license)?;
        match licenses.as_slice() {
            [] => unreachable!("resolve_licenses returns at least one license"),
            [single] => {
                std::fs::write(dir.join("LICENSE"), &single.text).io_err()?;
            }
            multiple => {
                let mut index = format!(
                    "This project is licensed under: {}\n\n\
                     The full text of each license is provided in the corresponding \
                     file listed below.\n\n",
                    cfg.package.license,
                );
                for lic in multiple {
                    let filename = format!("LICENSE-{}", lic.id);
                    std::fs::write(dir.join(&filename), &lic.text).io_err()?;
                    index.push_str(&format!("- {}: see {}\n", lic.name, filename));
                }
                std::fs::write(dir.join("LICENSE"), index).io_err()?;
            }
        }

        pb.set_message("Writing AUTHORS file...");
        if !cfg.package.authors.is_empty() {
            let authors_file = cfg
                .package
                .authors
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(dir.join("AUTHORS"), authors_file + "\n").io_err()?;
        }

        pb.set_message("Verifying go build...");
        CommandWrapper::new(
            Command::new("go")
                .args(["build", "./..."])
                .current_dir(&dir),
            "golang",
        )
        .progress(pb.clone())
        .run()
        .await?;

        *self.get_work_dir().await = Some(tmp);

        pb.finish_and_clear();

        Ok(())
    }

    async fn publish(&self, cfg: Config, pb: ProgressBar) -> Result<()> {
        if cfg.golang.is_none() {
            return Ok(());
        }
        let golang_config = cfg.golang.as_ref().ok_or_else(|| Error::MissingConfig {
            field: "golang.remote".into(),
            hint: indoc::indoc! {r#"
                Add the following to your Buffy.toml:
                [golang]
                remote = "github.com/your-user/your-repo-go.git"
            "#}
            .into(),
        })?;
        let guard = self.get_work_dir().await;
        let dir = guard
            .as_ref()
            .ok_or_else(|| Error::Internal("publish() run before build()".into()))?
            .path()
            .to_path_buf();
        let version = cfg.package.version.to_string();
        let tag = format!("v{version}");
        let remote = &golang_config.remote;
        let branch = &golang_config.branch;

        macro_rules! git {
            ($pb:expr, $dir:expr, $($arg:expr),+) => {
                CommandWrapper::new(
                    Command::new("git").args([$($arg),+]).current_dir($dir),
                    "golang",
                )
                .progress($pb.clone())
            };
        }

        pb.set_message("Initializing git repository...");
        git!(pb, &dir, "init", "-b", branch).run().await?;

        pb.set_message("Configuring remote...");
        let remote_result = git!(pb, &dir, "remote", "add", "origin", remote)
            .run()
            .await;
        if remote_result.is_err() {
            git!(pb, &dir, "remote", "set-url", "origin", remote)
                .run()
                .await?;
        }

        pb.set_message("Fetching existing files from remote...");
        let fetch_result = git!(pb, &dir, "fetch", "origin", branch)
            .env("GIT_TERMINAL_PROMPT", "0")
            .run()
            .await;

        if fetch_result.is_ok() {
            for file in &golang_config.keep {
                let result = git!(
                    pb,
                    &dir,
                    "checkout",
                    &format!("origin/{branch}"),
                    "--",
                    file.as_str()
                )
                .run()
                .await;

                if result.is_err() {
                    pb.suspend(|| {
                        eprintln!(
                            "{} {} not found on remote, skipping",
                            style("[~]").yellow().bold(),
                            style(file).dim(),
                        );
                    });
                }
            }
        }

        git!(pb, &dir, "add", ".").run().await?;
        git!(pb, &dir, "commit", "-m", &format!("release {tag}"))
            .run()
            .await?;

        pb.set_message(format!("Tagging {tag}..."));
        git!(pb, &dir, "tag", "-f", &tag).run().await?;

        pb.set_message(format!("Pushing {tag} to {branch}..."));
        git!(
            pb,
            &dir,
            "push",
            "--force",
            "origin",
            &format!("HEAD:{branch}")
        )
        .env("GIT_TERMINAL_PROMPT", "0")
        .run()
        .await?;

        git!(pb, &dir, "push", "--force", "origin", "--tags")
            .env("GIT_TERMINAL_PROMPT", "0")
            .run()
            .await?;

        pb.finish_with_message(format!("✓ Published {tag} → {remote}"));
        Ok(())
    }
}
