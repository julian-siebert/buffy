use crate::targets::context::TARGETS_DIRECTORY_PATH;

const GITIGNORE_PATH: &str = ".gitignore";

pub fn ensure_target_in_gitignore() -> Result<(), crate::io::Error> {
    if !crate::io::exists(GITIGNORE_PATH)? {
        crate::io::write(GITIGNORE_PATH, format!("{TARGETS_DIRECTORY_PATH}\n"))?;
        return Ok(());
    }

    let content = crate::io::read_to_string(GITIGNORE_PATH)?;

    if has_entry(&content, TARGETS_DIRECTORY_PATH) {
        return Ok(());
    }

    let mut updated = content;
    if !updated.is_empty() && !updated.ends_with('\n') {
        updated.push('\n');
    }

    updated.push_str(TARGETS_DIRECTORY_PATH);
    updated.push('\n');

    crate::io::write(GITIGNORE_PATH, updated)?;

    Ok(())
}

fn has_entry(content: &str, entry: &str) -> bool {
    content.lines().any(|line| {
        let trimmed = line.trim();
        // skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return false;
        }
        // strip leading slash and trailing slash for comparison
        // so "target", "/target", "target/", "/target/" all match
        let normalized = trimmed.trim_start_matches('/').trim_end_matches('/');
        normalized == entry
    })
}
