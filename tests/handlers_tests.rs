mod common;

use std::{
    env::set_current_dir,
    fs::{create_dir_all, read_to_string},
    path::{Path, PathBuf},
};

use anyhow::Result;
use cmd_lib::run_fun;
use common::setup_temp_dir_handle_init;
use ripcards::{
    cli::{NewCardArgs, SessionMethodArgs},
    config::Config,
    handlers::{handle_init, handle_new_card, handle_session_start},
    msgs::GIT_COMMIT_MSG_RIPC_INIT,
};
use tempfile::tempdir;

#[test]
fn test_handle_init() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    handle_init()?;
    let gitkeep_path_str = "ripc/sessions/.gitkeep";
    let config_path_str = "ripc/config.toml";

    let config_content = read_to_string(config_path_str)?;
    let config: Config = toml::from_str(&config_content)?;
    assert_eq!(config, Config::default());

    assert!(
        Path::new(".git").is_dir(),
        "Git repository must be initialized."
    );
    assert!(Path::new(gitkeep_path_str).is_file());
    assert!(Path::new(config_path_str).is_file());

    let out_git_status = run_fun!(git status --porcelain)?;
    assert!(!out_git_status.contains(gitkeep_path_str));
    assert!(!out_git_status.contains(config_path_str));

    let out_git_log = run_fun!(git log --oneline)?;
    assert!(out_git_log.contains(GIT_COMMIT_MSG_RIPC_INIT));

    Ok(())
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
    let proj_root = setup_temp_dir_handle_init()?;
    let args = NewCardArgs { path: None };
    handle_new_card(&args)?;
    assert_new_card_files(&proj_root);
    Ok(())
}

#[test]
fn test_handle_new_card_with_dot_path() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let args = NewCardArgs {
        path: Some(PathBuf::from(".")),
    };
    handle_new_card(&args)?;
    assert_new_card_files(&proj_root);
    Ok(())
}

#[test]
fn test_handle_new_card_with_some_path() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    let some_path = &proj_root.join("card/path");
    create_dir_all(some_path)?;
    let args = NewCardArgs {
        path: Some(some_path.clone()),
    };
    handle_new_card(&args)?;
    assert_new_card_files(some_path);
    Ok(())
}

#[test]
fn test_handle_session_start_must_git_root() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    let args = SessionMethodArgs { method: None };
    let res = handle_session_start(&args);
    assert!(
        res.is_err(),
        "ripc session start must return error if is not executed at the root of a git repository"
    );
    Ok(())
}

#[test]
fn test_handle_session_start() -> Result<()> {
    todo!()
}
