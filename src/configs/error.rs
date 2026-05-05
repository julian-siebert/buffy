use miette::{Diagnostic, NamedSource, SourceSpan};

use crate::configs::author::AuthorParseError;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] crate::io::Error),

    #[error("Failed to parse TOML in {path}")]
    #[diagnostic(
        code(config::toml_parse),
        help("Check the syntax around the highlighted location.")
    )]
    TomlParse {
        path: String,
        #[source_code]
        src: NamedSource<String>,
        #[label("{message}")]
        span: SourceSpan,
        message: String,
    },

    #[error(transparent)]
    #[diagnostic(transparent)]
    Author(#[from] AuthorParseError),

    #[error("Duplicate profile name `{name}`")]
    #[diagnostic(
        code(config::profile::duplicate),
        help("Profile names must be unique. Rename one of: {first} or {second}.")
    )]
    DuplicateProfile {
        name: String,
        first: String,
        second: String,
    },
}
