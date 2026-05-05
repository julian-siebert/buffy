use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Java {
    #[serde(rename = "maven")]
    Maven {
        group_id: String,
        artifact_id: String,
    },
}
