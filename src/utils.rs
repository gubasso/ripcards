mod find_cards;

use std::{
    env::{current_dir, set_current_dir},
    fs::{create_dir_all, write},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, bail, Result};

use crate::cli::NewCardArgs;

pub fn git_add_files<P: AsRef<Path>>(path: &[P]) -> Result<()> {
    let paths_str: Vec<String> = path
        .iter()
        .map(|p| p.as_ref().to_string_lossy().into_owned())
        .collect();

    let output = Command::new("git")
        .arg("add")
        .args(&paths_str)
        .output()
        .map_err(|e| anyhow!("Failed to execute 'git add' command: {}", e))?;

    if !output.status.success() {
        bail!(
            "'git add' command failed with status code: {}\nError: {}\nOutput: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout),
        );
    }

    Ok(())
}

pub fn write_file_contents<P, C>(path: P, contents: C) -> Result<()>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    let parent_dir = path.as_ref().parent().unwrap();
    create_dir(parent_dir)?;
    write(&path, contents).map_err(|e| {
        anyhow!(
            "Failed to write content to file '{}'. Error: {}.",
            path.as_ref().display(),
            e
        )
    })
}

pub fn create_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    create_dir_all(&path).map_err(|e| {
        anyhow!(
            "Failed to create directory '{}'. Error: {}",
            path.as_ref().display(),
            e
        )
    })
}

pub fn set_curr_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    create_dir(&path)?;
    set_current_dir(&path).map_err(|e| {
        anyhow!(
            "Failed to set current directory '{}'. Error: {}",
            path.as_ref().display(),
            e
        )
    })
}

pub fn get_relative_path(base: &Path, full_path: &Path) -> Result<PathBuf> {
    full_path
        .strip_prefix(base)
        .map_err(|e| {
            anyhow!(
                "get_relative_path: Failed to strip prefix.\n \
            base: {}\n \
            full_path: {}\n \
            e: {}",
                base.display(),
                full_path.display(),
                e
            )
        })
        .and_then(|rel_path| {
            if rel_path.as_os_str().is_empty() {
                bail!(
                    "get_relative_path: Resulting relative path is empty.\nbase: {}\nfull_path: {}",
                    base.display(),
                    full_path.display()
                )
            }
            Ok(rel_path.to_path_buf())
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

pub fn validate_relative_path(path: &str) -> Result<(), String> {
    if !Path::new(path).is_relative() {
        return Err(String::from("The path must be a relative path"));
    }
    Ok(())
}

pub fn get_handle_new_card_args() -> [NewCardArgs; 3] {
    [
        NewCardArgs { path: None },
        NewCardArgs {
            path: Some(PathBuf::from(".")),
        },
        NewCardArgs {
            path: Some(PathBuf::from("in/a/card/path")),
        },
    ]
}
