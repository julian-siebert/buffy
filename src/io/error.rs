use std::path::{Path, PathBuf};

use miette::Diagnostic;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error("Permission denied for {path}")]
    #[diagnostic(
        code(fs::permission_denied),
        help("Check the file permissions with `ls -l {}`.", path.display())
    )]
    PermissionDenied {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("File not found: {path}")]
    #[diagnostic(
        code(fs::not_found),
        help("Make sure the file exists and the path is correct.")
    )]
    NotFound {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Expected {path} to be a directory, but it is a file")]
    #[diagnostic(
        code(fs::not_a_directory),
        help("Remove the file at {} or rename it, then create a directory there.", path.display())
    )]
    NotADirectory { path: PathBuf },

    #[error("I/O error at {path}")]
    #[diagnostic(code(fs::io))]
    Other {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub(super) fn from_io(path: &Path, source: std::io::Error) -> Error {
    let path = path.to_owned();
    match source.kind() {
        std::io::ErrorKind::NotFound => Error::NotFound { path, source },
        std::io::ErrorKind::PermissionDenied => Error::PermissionDenied { path, source },
        _ => Error::Other { path, source },
    }
}
