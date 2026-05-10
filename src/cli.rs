use semver::Version;

/// Buffy - generate and publish protobuf libraries for multiple languages.
///
/// Buffy reads `Buffy.toml` and the profiles in `.buffy/`, generates
/// language-specific code from your `.proto` files, and (optionally)
/// publishes the resulting packages to their respective registries.
///
/// Without any subcommand, Buffy builds all configured profiles in parallel.
/// With `--publish`, it also publishes them after a successful build.
#[derive(Debug, clap::Parser)]
#[command(name = "buffy", version)]
pub struct Cli {
    /// Publish each profile after a successful build.
    ///
    /// Each profile is published to the registry configured in its
    /// `.buffy/<name>.toml` file (e.g. crates.io for Rust, Maven Central
    /// for Java, npm for JavaScript/TypeScript, or a Git remote).
    ///
    /// Required environment variables depend on the target — Buffy will
    /// emit a clear error if any are missing.
    #[arg(short, long, default_value = "false")]
    pub publish: bool,

    /// Override the version from `Buffy.toml` for this run.
    ///
    /// Useful in CI pipelines where the version is derived from a Git tag
    /// rather than committed into `Buffy.toml`. Must be a valid SemVer
    /// version (e.g. `1.2.3`, `0.1.0-rc.1`).
    #[arg(long)]
    pub publish_version: Option<Version>,

    /// Print full command output (stdout) from invoked tools.
    ///
    /// By default, only stderr is shown to keep the output focused on
    /// warnings and errors. Use this when debugging build failures or
    /// when you want to see the full progress of `mvn`, `cargo`, `npm`,
    /// etc.
    #[arg(long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcommands for non-build operations.
#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    /// Check that all required tools are installed for the configured profiles.
    ///
    /// Verifies the toolchain (e.g. `protoc`, `cargo`, `mvn`, `go`, `npm`)
    /// without running any builds. Returns non-zero if any tool is missing,
    /// with diagnostic help on how to install it.
    Check,
}
