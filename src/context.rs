use std::path::{Path, PathBuf};

use anyhow::{bail, Result};

use crate::msgs::error::ERROR_MSG_NOT_PROJECT_ROOT;

#[derive(Debug)]
pub struct Context {
    wd: PathBuf,
}

impl Context {
    pub fn new(wd: PathBuf) -> Result<Self> {
        let is_git_dir = wd.join(".git").is_dir();
        if !is_git_dir {
            bail!(ERROR_MSG_NOT_PROJECT_ROOT);
        }
        Ok(Self { wd })
    }
    pub fn wd(&self) -> &Path {
        &self.wd
    }
}
