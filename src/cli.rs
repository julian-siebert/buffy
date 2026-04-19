use std::path::PathBuf;

use semver::Version;

/// Generate and publish gRPC/Protobuf stubs for multiple languages
/// from a single Buffy.toml configuration file.
#[derive(Debug, clap::Parser)]
#[command(
    name = "buffy",
    about = "Generate and publish gRPC/Protobuf stubs for Go, Java, and Rust",
    long_about = "buffy reads your Buffy.toml and runs protoc with the correct \
                  plugins for each configured language, then publishes the \
                  generated packages to their respective registries."
)]
pub struct Cli {
    /// Override the version defined in Buffy.toml.
    ///
    /// Useful in CI where the version is derived from a Git tag:
    ///   buffy --version 1.2.3 --publish
    #[arg(short, long)]
    pub version: Option<Version>,

    /// Publish generated stubs after building.
    ///
    /// Requires the appropriate credentials to be set via environment
    /// variables. See the README for details.
    #[arg(short, long, default_value = "false")]
    pub publish: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Create a new Buffy.toml in the current or specified directory.
    ///
    /// Generates a minimal configuration file with the given project name.
    /// Edit the file to configure target languages and registries.
    ///
    /// Examples:
    ///   buffy init myservice
    ///   buffy init myservice --path ./projects/myservice
    Init {
        /// Name of the project. Used as the package name in Buffy.toml.
        name: String,

        /// Directory to create Buffy.toml in. Defaults to the current directory.
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// Validate Buffy.toml and check that all required tools are available.
    ///
    /// Checks:
    ///   - Buffy.toml exists and is valid
    ///   - protoc is installed and on PATH
    ///   - Language-specific plugins are installed (protoc-gen-go, protoc-gen-prost, etc.)
    ///   - Source proto files exist at the configured path
    Check,
}
