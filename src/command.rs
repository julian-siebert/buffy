use std::process::Stdio;

use console::style;
use indicatif::ProgressBar;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::error::{Error, IoResultExt, Result};

pub struct CommandWrapper<'a> {
    cmd: &'a mut tokio::process::Command,
    prefix: &'a str,
    pb: Option<ProgressBar>,
}

impl<'a> CommandWrapper<'a> {
    pub fn new(cmd: &'a mut tokio::process::Command, prefix: &'a str) -> Self {
        Self {
            cmd,
            prefix,
            pb: None,
        }
    }

    pub fn progress(mut self, pb: ProgressBar) -> Self {
        self.pb = Some(pb);
        self
    }

    pub fn env(self, key: &str, val: &str) -> Self {
        self.cmd.env(key, val);
        self
    }

    pub async fn run(self) -> Result<()> {
        let cmd_name = format!("{:?}", self.cmd.as_std().get_program());

        let mut child = self
            .cmd
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .io_err()?;

        let stderr = child.stderr.take().unwrap();
        let prefix = self.prefix.to_string();

        let pb = self.pb.clone();

        let cmd_display = cmd_name.clone();

        let err_task = tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();

            while let Ok(Some(line)) = lines.next_line().await {
                match &pb {
                    Some(pb) => pb.suspend(|| {
                        eprintln!(
                            "{} {} {} {line}",
                            style("[~]").yellow().bold(),
                            style(prefix.to_uppercase()).bold(),
                            style(&cmd_display).cyan()
                        )
                    }),
                    None => eprintln!("[{prefix}] {line}"),
                };
            }
        });

        let (_, status) = tokio::join!(err_task, child.wait());
        let status = status.io_err()?;

        if !status.success() {
            return Err(Error::CommandFailed {
                cmd: cmd_name,
                code: status.code().unwrap_or(-1),
                help: None,
            });
        }

        Ok(())
    }
}
