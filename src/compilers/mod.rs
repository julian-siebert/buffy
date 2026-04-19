use std::path::PathBuf;

use crate::error::Result;

pub mod golang;
pub mod java;
pub mod rust;

pub(super) fn collect_proto_files(source: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("proto") {
            files.push(path);
        } else if path.is_dir() {
            files.extend(collect_proto_files(&path)?);
        }
    }
    Ok(files)
}
