use std::{
    env::set_current_dir,
    fs::{create_dir, create_dir_all},
    path::{Path, PathBuf},
};

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
    create_dir(".git")?;
    handle_init()?;
    let ripc_dir = temp_dir.path().join("ripc");
    assert!(ripc_dir.is_dir());
    let sessions_dir = ripc_dir.join("sessions");
    assert!(sessions_dir.is_dir());
    let config_file = ripc_dir.join("config.toml");
    assert!(config_file.is_file());
    Ok(())
}

#[test]
fn test_handle_init_must_git_and_root() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    let handle_init_res = handle_init();
    assert!(
        handle_init_res.is_err(),
        "ripc init must return error if is not executed at the root of a git repository"
    );
    Ok(())
}

fn setup_temp_dir() -> Result<TempDir> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    create_dir(".git")?;
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

#[test]
fn test_handle_new_card_with_dot_path() -> Result<()> {
    let temp_dir = setup_temp_dir()?;
    let args = NewCardArgs {
        path: Some(PathBuf::from(".")),
    };
    handle_new_card(&args)?;
    let curr_dir = temp_dir.path();
    assert_new_card_files(curr_dir);
    Ok(())
}

#[test]
fn test_handle_new_card_with_some_path() -> Result<()> {
    let temp_dir = setup_temp_dir()?;
    let some_path = temp_dir.path().join("card/path");
    create_dir_all(&some_path)?;
    let args = NewCardArgs {
        path: Some(some_path.clone()),
    };
    handle_new_card(&args)?;
    assert_new_card_files(&some_path);
    Ok(())
}

#[test]
fn test_handle_new_card_with_invalid_path() -> Result<()> {
    let temp_dir = setup_temp_dir()?;
    let invalid_dir = temp_dir.path().join("card/path");
    let args = NewCardArgs {
        path: Some(invalid_dir),
    };
    let new_card_res = handle_new_card(&args);
    assert!(
        new_card_res.is_err(),
        "If path passed does not exists, it must return an error."
    );
    Ok(())
}
