use std::{fs::create_dir, path::PathBuf};

use anyhow::Result;
use ripcards::{handlers::handle_init, utils::set_current_directory};
use tempfile::tempdir;

pub fn setup_temp_dir_handle_init() -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    set_current_directory(&temp_dir)?;
    create_dir(".git")?;
    handle_init()?;
    Ok(temp_dir.into_path())
}
