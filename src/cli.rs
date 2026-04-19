use std::path::PathBuf;

use semver::Version;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub version: Option<Version>,

    #[arg(short, long, default_value = "false")]
    pub publish: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Init {
        name: String,

        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    Check,
}
