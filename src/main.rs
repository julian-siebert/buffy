use std::sync::Arc;

use clap::Parser;
use console::style;

use crate::{
    cli::Cli,
    compiler::Compiler,
    compilers::{golang::GolangCompiler, java::JavaCompiler, rust::RustCompiler},
    config::Config,
};

mod build;
mod cli;
pub mod command;
pub mod compiler;
mod compilers;
pub mod config;
pub mod configs;
#[allow(unused_assignments)]
pub mod error;
mod gitignore;
mod init;
pub mod io;
pub mod license;
mod publish;
pub mod targets;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            cli::Commands::Init { name, path } => {
                init::init(&name, path).map_err(miette::Report::new)?;
                return Ok(());
            }
            cli::Commands::Check => {
                println!(
                    "{} {}",
                    style("[-]").green().bold(),
                    style("Cheking").bold(),
                );

                let config = Config::load()?;

                let grpc = config.package.grpc;

                if config.golang.is_some() {
                    GolangCompiler::new(grpc)?;
                }

                if config.java.is_some() {
                    JavaCompiler::new()?;
                }

                if config.rust.is_some() {
                    RustCompiler::new(grpc)?;
                }

                println!(
                    "{} {}",
                    style("[+]").green().bold(),
                    style("Check successful. Installation seems fine.").bold(),
                );

                return Ok(());
            }
        };
    }

    dotenvy::dotenv().ok();

    let mut config = Config::load()?;

    if config.no_targets() {
        println!(
            "{} {}",
            style("[~]").bright().bold(),
            style("No targets configured in Buffy.toml").bold(),
        );
        return Ok(());
    }

    if let Some(version) = cli.publish_version {
        config.package.version = version;
    }

    let mut compilers: Vec<Arc<dyn Compiler>> = Vec::new();

    let grpc = config.package.grpc;

    if config.golang.is_some() {
        compilers.push(Arc::new(GolangCompiler::new(grpc)?));
    }

    if config.java.is_some() {
        compilers.push(Arc::new(JavaCompiler::new()?));
    }

    if config.rust.is_some() {
        compilers.push(Arc::new(RustCompiler::new(grpc)?));
    }

    build::build(config.clone(), compilers.clone()).await?;

    if cli.publish {
        publish::publish(config, compilers).await?;
    }

    Ok(())
}
