use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Golang {
    #[serde(rename = "git")]
    Git {
        module: String,
        remote: String,
        #[serde(default = "default_branch")]
        branch: String,
        #[serde(default = "default_keep")]
        keep: Vec<String>,
    },
}

fn default_branch() -> String {
    "main".into()
}

fn default_keep() -> Vec<String> {
    vec![]
}
