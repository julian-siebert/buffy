use std::{ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

pub use crate::configs::profiles::{
    golang::Golang, java::Java, javascript::JavaScript, kotlin::Kotlin, rust::Rust,
    typescript::TypeScript,
};

mod golang;
mod java;
mod javascript;
mod kotlin;
mod rust;
mod typescript;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Profile {
    #[serde(rename = "golang")]
    Golang(Golang),
    #[serde(rename = "java")]
    Java(Java),
    #[serde(rename = "kotlin")]
    Kotlin(Kotlin),
    #[serde(rename = "javascript")]
    JavaScript(JavaScript),
    #[serde(rename = "rust")]
    Rust(Rust),
    #[serde(rename = "typescript")]
    TypeScript(TypeScript),
}

#[derive(Debug, Clone)]
pub struct NamedProfile(Arc<NamedProfileInner>);

#[derive(Debug)]
pub struct NamedProfileInner {
    pub name: String,
    pub profile: Profile,
}

impl NamedProfile {
    pub fn new(name: String, profile: Profile) -> Self {
        Self(Arc::new(NamedProfileInner { name, profile }))
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn kind(&self) -> &Profile {
        &self.0.profile
    }
}

impl Deref for NamedProfile {
    type Target = Profile;

    fn deref(&self) -> &Self::Target {
        &self.0.profile
    }
}
