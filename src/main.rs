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
#[allow(unused_assignments)]
pub mod error;
mod publish;

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    if let Some(_command) = cli.command {
        println!("Not implemented yet");
        return Ok(());
    }

    dotenv::dotenv().ok();

    let mut config = Config::load()?;

    if config.no_targets() {
        println!(
            "{} {}",
            style("[~]").bright().bold(),
            style("No targets configured in Buffy.toml").bold(),
        );
        return Ok(());
    }

    if let Some(version) = cli.version {
        config.package.version = version;
    }

    let mut compilers: Vec<Arc<dyn Compiler>> = Vec::new();

    if config.golang.is_some() {
        compilers.push(Arc::new(GolangCompiler::new(config.grpc)?));
    }

    if config.java.is_some() {
        compilers.push(Arc::new(JavaCompiler::new()?));
    }

    if config.rust.is_some() {
        compilers.push(Arc::new(RustCompiler::new(config.grpc)?));
    }

    build::build(config.clone(), compilers.clone()).await?;

    if cli.publish {
        publish::publish(config, compilers).await?;
    }

    Ok(())
}
