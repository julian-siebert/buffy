use std::{path::PathBuf, sync::Arc};

use miette::{NamedSource, SourceSpan};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub package: Package,
    #[serde(default)]
    pub source: Source,

    pub golang: Option<Golang>,

    pub java: Option<Java>,

    pub rust: Option<Rust>,
}

impl Config {
    pub fn load() -> Result<Config> {
        let content = std::fs::read_to_string("Buffy.toml").map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::BuffyTomlNotFound
            } else {
                Error::Io(Arc::new(e))
            }
        })?;

        toml::from_str(&content).map_err(|e| {
            let span = e.span().map(|r| SourceSpan::from(r));

            Error::BuffyTomlDeserialization {
                message: e.message().to_string(),
                src: NamedSource::new("Buffy.toml", content.clone()),
                span,
            }
        })
    }

    pub fn no_targets(&self) -> bool {
        self.golang.is_none() && self.rust.is_none() && self.java.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: String,
    pub version: Version,
    pub license: String,
    pub authors: Vec<Author>,
    pub homepage: String,

    #[serde(default = "grpc_default")]
    pub grpc: bool,
}

fn grpc_default() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
}

impl std::fmt::Display for Author {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.email {
            Some(e) => write!(f, "{} <{}>", self.name, e),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    #[serde(default = "default_source_path")]
    pub path: PathBuf,
}

impl Default for Source {
    fn default() -> Self {
        Self {
            path: default_source_path(),
        }
    }
}

fn default_source_path() -> PathBuf {
    PathBuf::from("./src")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Golang {
    pub module: String,
    pub remote: String,
    #[serde(default = "default_branch")]
    pub branch: String,
    #[serde(default = "default_keep")]
    pub keep: Vec<String>,
}

fn default_branch() -> String {
    "main".into()
}

fn default_keep() -> Vec<String> {
    vec![]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Java {
    pub group_id: String,
    pub artifact_id: String,
    pub url: String,
    pub scm: JavaScm,

    /// If unset, Maven central api will be used
    pub protobuf_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaScm {
    pub connection: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rust {
    pub name: String,
    #[serde(default = "default_edition")]
    pub edition: String,
    #[serde(default = "default_registry")]
    pub registry: String,

    pub documentation: String,

    pub repository: String,

    pub prost_version: Option<String>,

    pub tonic_version: Option<String>,
}

fn default_edition() -> String {
    "2021".into()
}

fn default_registry() -> String {
    "crates-io".into()
}
