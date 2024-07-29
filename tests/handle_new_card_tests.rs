mod common;

use std::{
    env::set_current_dir,
    fs::create_dir_all,
    path::{Path, PathBuf},
};

use anyhow::Result;
use common::setup_temp_dir_handle_init;
use ripcards::{cli::NewCardArgs, handlers::handle_new_card};
use tempfile::tempdir;

fn assert_new_card_files(card_dir: &Path) {
    assert!(card_dir.is_dir(), "Card dir must exists.");
    assert!(
        card_dir.join("ripcard.toml").is_file(),
        "ripcard.toml file must exist"
    );
    assert!(
        card_dir.join("question.md").is_file(),
        "question.md file must exist"
    );
    assert!(
        card_dir.join("answer.md").is_file(),
        "answer.md file must exist"
    );
}

#[test]
fn test_handle_new_card_proj_root() -> Result<()> {
    let _proj_root = setup_temp_dir_handle_init()?;
    let args = NewCardArgs { path: None };
    let res = handle_new_card(&args);
    assert!(
        res.is_err(),
        "Must return err if card is created at the project root."
    );
    Ok(())
}

#[test]
fn test_handle_new_card_proj_not_initialized() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    let some_card_path = PathBuf::from("some/card");
    let args = NewCardArgs {
        path: Some(some_card_path.clone()),
    };
    let res = handle_new_card(&args);
    assert!(
        res.is_err(),
        "New card command must return error: RipCards project is not initialized. Didn't find the ripc root dir."
    );
    Ok(())
}

#[test]
fn test_handle_new_card_input_path() -> Result<()> {
    let _proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = PathBuf::from("some/card");
    let args = NewCardArgs {
        path: Some(some_card_path.clone()),
    };
    handle_new_card(&args)?;
    assert_new_card_files(&some_card_path);
    Ok(())
}

#[test]
fn test_handle_new_card_input_dot_path() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = proj_root.join("some/card/path");
    create_dir_all(&some_card_path)?;
    set_current_dir(&some_card_path)?;
    let args = NewCardArgs {
        path: Some(PathBuf::from(".")),
    };
    handle_new_card(&args)?;
    assert_new_card_files(&some_card_path);
    Ok(())
}

#[test]
fn test_handle_new_card_input_none() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = proj_root.join("some/other/card/path");
    create_dir_all(&some_card_path)?;
    set_current_dir(&some_card_path)?;
    let args = NewCardArgs { path: None };
    handle_new_card(&args)?;
    assert_new_card_files(&some_card_path);
    Ok(())
}
