use std::{fs::create_dir, path::PathBuf};

use anyhow::Result;
use ripcards::{handlers::handle_init, utils::set_curr_dir};
use tempfile::tempdir;

pub fn setup_temp_dir_handle_init() -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    set_curr_dir(&temp_dir)?;
    create_dir(".git")?;
    handle_init()?;
    Ok(temp_dir.into_path())
}
