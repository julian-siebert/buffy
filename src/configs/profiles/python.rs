use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Python {
    #[serde(rename = "pypi")]
    Pypi(Pypi),
    #[serde(rename = "git")]
    Git(Git),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pypi {
    pub name: String,
    /// e.g. "https://upload.pypi.org/legacy/" or "https://test.pypi.org/legacy/"
    #[serde(default = "default_repository")]
    pub repository_url: String,
    pub repository: String,
    pub homepage: Option<String>,
    #[serde(default = "default_grpc")]
    pub grpc: bool,
    pub protobuf_version: Option<String>,
    pub grpcio_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Git {
    pub name: String,
    pub remote: String,
    pub branch: String,
    pub repository: String,
    pub homepage: Option<String>,
    #[serde(default = "default_grpc")]
    pub grpc: bool,
    pub protobuf_version: Option<String>,
    pub grpcio_version: Option<String>,
    #[serde(default)]
    pub keep: Vec<String>,
}

fn default_repository() -> String {
    "https://upload.pypi.org/legacy/".to_string()
}

fn default_grpc() -> bool {
    true
}
