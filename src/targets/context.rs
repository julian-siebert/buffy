use std::{ops::Deref, path::PathBuf, sync::Arc};

use crate::{
    config::Package,
    configs::{Error, profiles::NamedProfile},
};

pub const TARGETS_DIRECTORY_PATH: &str = "target";

#[derive(Clone)]
pub struct Context(Arc<ContextInner>);

pub struct ContextInner {
    pub package: Package,
    pub profile: NamedProfile,
    pub target_path: PathBuf,
}

impl Context {
    pub fn new(package: Package, profile: NamedProfile) -> Result<Self, Error> {
        let target_path = PathBuf::from(TARGETS_DIRECTORY_PATH).join(profile.name());

        if crate::io::exists(&target_path)? {
            crate::io::remove_dir_all(&target_path)?;
        }

        crate::io::create_dir_all(&target_path)?;

        Ok(Self(Arc::new(ContextInner {
            package,
            profile,
            target_path,
        })))
    }
}

impl Deref for Context {
    type Target = ContextInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
