use std::sync::Arc;

use miette::{Diagnostic, NamedSource, SourceSpan};

#[derive(Debug, Clone, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error("Path error: {0}")]
    #[diagnostic(code(buffy::path), help("Is the program installed in PATH?"))]
    Which(#[from] which::Error),

    #[error("IO error: {0}")]
    #[diagnostic(code(buffy::io))]
    Io(#[from] Arc<std::io::Error>),

    #[error("Buffy.toml deserialization error: {message}")]
    #[diagnostic(code(buffy::config::parse))]
    BuffyTomlDeserialization {
        message: String,
        #[source_code]
        src: NamedSource<String>,
        #[label("here")]
        span: Option<SourceSpan>,
    },

    #[error("Buffy.toml not found")]
    #[diagnostic(
        code(buffy::config::missing),
        help("Run `buffy init` to initialize a new Buffy project with `Buffy.toml`")
    )]
    BuffyTomlNotFound,

    #[error("Missing configuration: `{field}`")]
    #[diagnostic(code(buffy::config::missing_field))]
    MissingConfig {
        field: String,
        #[help]
        hint: String,
    },

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Prozess {cmd} beendet mit Code {code}")]
    #[diagnostic(code(buffy::command::failed))]
    CommandFailed {
        cmd: String,
        code: i32,
        #[help]
        help: Option<String>,
    },

    #[error("{count} compiler(s) failed")]
    #[diagnostic(code(buffy::build::failed), help("See the errors above for details."))]
    BuildFailed { count: usize },

    #[error("Missing program: {program}")]
    #[diagnostic(help("Is it installed in PATH?"))]
    MissingProgram { program: String },

    #[error("Invalid SPDX expression: {expr}")]
    SpdxParse { expr: String },
}
pub type Result<T> = std::result::Result<T, Error>;

pub trait IoResultExt<T> {
    fn io_err(self) -> Result<T>;
}

impl<T> IoResultExt<T> for std::result::Result<T, std::io::Error> {
    fn io_err(self) -> Result<T> {
        self.map_err(|e| Error::Io(Arc::new(e)))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(Arc::new(e))
    }
}
