use std::{
    collections::HashSet,
    env::current_dir,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use walkdir::WalkDir;

use crate::{methods::Method, msgs::error::ERROR_MSG_NOT_PROJECT_ROOT};

pub fn assert_git_repo_root() -> Result<()> {
    if !current_dir()?.join(".git").is_dir() {
        bail!(ERROR_MSG_NOT_PROJECT_ROOT);
    }
    Ok(())
}

pub fn is_repo_initialized() -> Result<bool> {
    let current_dir = current_dir()?;
    let git_dir = current_dir.join(".git");
    let ripc_dir = current_dir.join("ripc");
    let config_file = ripc_dir.join("config.toml");
    let sessions_dir = ripc_dir.join("sessions");
    let is_initialized =
        git_dir.is_dir() && ripc_dir.is_dir() && config_file.is_file() && sessions_dir.is_dir();
    Ok(is_initialized)
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
