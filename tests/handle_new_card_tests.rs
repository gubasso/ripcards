mod common;

use std::path::{Path, PathBuf};

use anyhow::Result;
use cmd_lib::run_fun;
use common::setup_temp_dir_handle_init;
use ripcards::{
    cli::NewCardArgs,
    handlers::handle_new_card,
    msgs::git_commit_msg_ripc_new,
    utils::{create_directory, set_curr_dir},
};
use tempfile::tempdir;

fn assert_new_card_files_and_git(card_dir: &Path) -> Result<()> {
    assert!(card_dir.is_dir(), "Card dir must exists.");
    let out_files_not_added = run_fun!(git status --porcelain)?;
    let out_git_log = run_fun!(git log --oneline)?;

    let new_card_files = ["ripcard.toml", "question.md", "answer.md"];

    for file in new_card_files.into_iter() {
        let path = card_dir.join(file);
        let path_str = path.to_str().unwrap();
        assert!(path.is_file(), "{} file must exist", file);
        assert!(!out_files_not_added.contains(path_str));
        assert!(out_git_log.contains(&git_commit_msg_ripc_new(path_str)));
    }

    Ok(())
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
    set_curr_dir(&temp_dir)?;
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
    let proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = PathBuf::from("some/card");
    let args = NewCardArgs {
        path: Some(some_card_path.clone()),
    };
    handle_new_card(&args)?;
    let new_card_abs_path = proj_root.join(&some_card_path);

    assert_new_card_files_and_git(&some_card_path)?;
    Ok(())
}

#[test]
fn test_handle_new_card_input_dot_path() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = proj_root.join("some/card/path");
    create_directory(&some_card_path)?;
    set_curr_dir(&some_card_path)?;
    let args = NewCardArgs {
        path: Some(PathBuf::from(".")),
    };
    handle_new_card(&args)?;
    assert_new_card_files_and_git(&some_card_path)?;
    Ok(())
}

#[test]
fn test_handle_new_card_input_none() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let some_card_path = proj_root.join("some/other/card/path");
    create_directory(&some_card_path)?;
    set_curr_dir(&some_card_path)?;
    let args = NewCardArgs { path: None };
    handle_new_card(&args)?;
    assert_new_card_files_and_git(&some_card_path)?;
    Ok(())
}
