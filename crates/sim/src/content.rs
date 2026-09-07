//! Loading storylets from disk or from the bundled set.

use crate::storylet::{ContentError, Storylet};
use std::path::Path;

/// Every storylet bundled into the binary, as (file name, text).
pub const BUNDLED: &[(&str, &str)] = &include!(concat!(env!("OUT_DIR"), "/bundled_storylets.rs"));

/// Every project definition bundled into the binary, as (file name, text).
pub const BUNDLED_PROJECTS: &[(&str, &str)] =
    &include!(concat!(env!("OUT_DIR"), "/bundled_projects.rs"));

/// Parses the bundled storylets.
pub fn bundled() -> Result<Vec<Storylet>, ContentError> {
    let mut out = Vec::with_capacity(BUNDLED.len());
    for (file, text) in BUNDLED {
        out.push(Storylet::from_toml(file, text)?);
    }
    check_unique(&out)?;
    Ok(out)
}

/// Loads every `*.toml` under a directory.
pub fn load_dir(dir: &Path) -> Result<Vec<Storylet>, ContentError> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| ContentError::Invalid {
            file: dir.display().to_string(),
            message: e.to_string(),
        })?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    let mut out = Vec::new();
    for path in files {
        let name = path
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
        let text = std::fs::read_to_string(&path).map_err(|e| ContentError::Invalid {
            file: name.clone(),
            message: e.to_string(),
        })?;
        out.push(Storylet::from_toml(&name, &text)?);
    }
    check_unique(&out)?;
    Ok(out)
}

fn check_unique(storylets: &[Storylet]) -> Result<(), ContentError> {
    let mut seen = std::collections::BTreeSet::new();
    for s in storylets {
        if !seen.insert(&s.id) {
            return Err(ContentError::Invalid {
                file: s.id.clone(),
                message: "duplicate storylet id".into(),
            });
        }
    }
    Ok(())
}
