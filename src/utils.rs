use std::{
    collections::HashSet,
    env::{current_dir, set_current_dir},
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use walkdir::WalkDir;

use crate::{methods::Method, msgs::error::ERROR_MSG_NOT_PROJECT_ROOT};

pub fn write_file_contents<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    write(&path, contents).map_err(|e| {
        anyhow!(
            "Failed to write content to file '{}'. Error: {}.",
            path.as_ref().display(),
            e
        )
    })
}

pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    create_dir_all(&path).map_err(|e| {
        anyhow!(
            "Filed to create directory '{}'. Error: {}",
            path.as_ref().display(),
            e
        )
    })
}

pub fn set_curr_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    set_current_dir(&path).map_err(|e| {
        anyhow!(
            "Filed to set current directory '{}'. Error: {}",
            path.as_ref().display(),
            e
        )
    })
}

pub fn get_relative_path(base: &Path, full_path: &Path) -> Option<PathBuf> {
    full_path.strip_prefix(base).ok().and_then(|p| {
        if p.as_os_str().is_empty() {
            None
        } else {
            Some(p.to_path_buf())
        }
    })
}

pub fn is_ripc_root(path: &Path) -> bool {
    path.join(".git").is_dir()
        && path.join("ripc/sessions").is_dir()
        && path.join("ripc/config.toml").is_file()
}

pub fn find_ripc_root() -> Result<PathBuf> {
    let mut curr_dir = current_dir()?;
    loop {
        if is_ripc_root(&curr_dir) {
            return Ok(curr_dir);
        }
        curr_dir = curr_dir
            .parent()
            .ok_or_else(|| anyhow!("Unable to find the RipCards project root."))?
            .to_path_buf();
    }
}

pub fn find_cards(root: &Path, method: Option<Method>) -> Result<HashSet<PathBuf>> {
    let mut matching_dirs = HashSet::new();
    let method_str = method.unwrap_or(Method::Leitner(())).to_string();
    let method_table = format!("[method.{}]", method_str);
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name() == "ripcard.toml")
    {
        let content = read_to_string(entry.path())?;
        println!("{}", content);
        if content.contains(&method_table) {
            if let Some(parent_dir) = entry.path().parent() {
                matching_dirs.insert(parent_dir.to_path_buf());
            }
        }
    }
    Ok(matching_dirs)
}

// remove
pub fn assert_git_repo_root() -> Result<()> {
    if !current_dir()?.join(".git").is_dir() {
        bail!(ERROR_MSG_NOT_PROJECT_ROOT);
    }
    Ok(())
}
