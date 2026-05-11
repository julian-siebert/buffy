use std::{ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::configs::profiles::{
    golang::Golang, java::Java, javascript::JavaScript, kotlin::Kotlin, python::Python, rust::Rust,
    typescript::TypeScript,
};

pub mod golang;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod python;
pub mod rust;
pub mod typescript;

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
    #[serde(rename = "python")]
    Python(Python),
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
