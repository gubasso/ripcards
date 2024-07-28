use std::{env::set_current_dir, fs::create_dir, path::PathBuf};

use anyhow::Result;
use ripcards::handlers::handle_init;
use tempfile::tempdir;

pub fn setup_temp_dir_handle_init() -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    create_dir(".git")?;
    handle_init()?;
    Ok(temp_dir.into_path())
}
