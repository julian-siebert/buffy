use std::collections::HashMap;

use miette::{NamedSource, SourceSpan};

pub use crate::configs::error::Error;
pub use crate::configs::main::Main;
use crate::configs::profiles::{NamedProfile, Profile};

mod error;
mod main;

pub mod author;

pub mod profiles;

const MAIN_CONFIG_PATH: &str = "Buffy.toml";
const PROFILE_CONFIGS_DIRECTORY_PATH: &str = ".buffy";

pub fn read_main() -> Result<Main, Error> {
    let content = crate::io::read_to_string(MAIN_CONFIG_PATH)?;

    parse_toml(MAIN_CONFIG_PATH, content)
}

pub fn read_profiles() -> Result<Vec<NamedProfile>, Error> {
    if !crate::io::exists(PROFILE_CONFIGS_DIRECTORY_PATH)? {
        return Ok(Vec::new());
    }

    let mut seen: HashMap<String, String> = HashMap::new();
    let mut profiles = Vec::new();

    crate::io::ensure_dir(PROFILE_CONFIGS_DIRECTORY_PATH)?;

    for entry in crate::io::read_dir(PROFILE_CONFIGS_DIRECTORY_PATH)? {
        let path = entry?;

        // skipping subdirectories and non-toml files
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let path_str = path.to_string_lossy().into_owned();

        if let Some(first_path) = seen.get(&name) {
            return Err(Error::DuplicateProfile {
                name,
                first: first_path.clone(),
                second: path_str,
            });
        }

        let content = crate::io::read_to_string(&path)?;
        let profile: Profile = parse_toml(&path_str, content)?;

        seen.insert(name.clone(), path_str);
        profiles.push(NamedProfile::new(name, profile));
    }

    Ok(profiles)
}

fn parse_toml<T: serde::de::DeserializeOwned>(path: &str, content: String) -> Result<T, Error> {
    toml::from_str(&content).map_err(|e| {
        let span = e
            .span()
            .map(|r| SourceSpan::from((r.start, r.end - r.start)))
            .unwrap_or_else(|| SourceSpan::from((0, 0)));

        Error::TomlParse {
            path: path.to_string(),
            src: NamedSource::new(path, content),
            span,
            message: e.message().to_string(),
        }
    })
}

#[cfg(test)]
mod tests {

    use crate::configs::profiles::Golang;

    use super::*;

    #[test]
    fn parse_golang_git_profile() {
        let toml_content = r#"
            [golang.git]
            module = "github.com/foo/bar"
            remote = "github.com/foo/bar.git"
            branch = "main"
        "#;

        let profile: Profile = toml::from_str(toml_content).unwrap();
        match profile {
            Profile::Golang(Golang::Git { module, .. }) => {
                assert_eq!(module, "github.com/foo/bar");
            }
            _ => panic!("wrong variant"),
        }
    }
}
