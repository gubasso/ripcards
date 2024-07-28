use std::{env::set_current_dir, path::Path};

use anyhow::Result;
use ripcards::{
    cli::NewCardArgs,
    handlers::{handle_init, handle_new_card},
};
use tempfile::{tempdir, TempDir};

mod common;

#[test]
fn test_handle_init() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    handle_init()?;
    let ripc_dir = temp_dir.path().join("ripc");
    assert!(ripc_dir.is_dir());
    let sessions_dir = ripc_dir.join("sessions");
    assert!(sessions_dir.is_dir());
    let config_file = ripc_dir.join("config.toml");
    assert!(config_file.is_file());
    Ok(())
}

fn setup_temp_dir() -> Result<TempDir> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    handle_init()?;
    Ok(temp_dir)
}

fn assert_new_card_files(curr_dir: &Path) {
    let ripcard_file = curr_dir.join("ripcard.toml");
    assert!(ripcard_file.is_file());
    let question_file = curr_dir.join("question.md");
    assert!(question_file.is_file());
    let answer_file = curr_dir.join("answer.md");
    assert!(answer_file.is_file());
}

#[test]
fn test_handle_new_card_with_none_path() -> Result<()> {
    let temp_dir = setup_temp_dir()?;
    let args = NewCardArgs { path: None };
    handle_new_card(&args)?;
    let curr_dir = temp_dir.path();
    assert_new_card_files(curr_dir);
    Ok(())
}
