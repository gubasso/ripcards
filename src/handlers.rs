use std::{
    env::current_dir,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use anyhow::{bail, Result};
use toml::to_string;

use crate::{
    cards::Card,
    cli::{NewCardArgs, SessionMethodArgs, SessionProgressArgs},
    config::Config,
    msgs::error::ERROR_MSG_NOT_PROJECT_ROOT,
};

fn assert_git_repo_root() -> Result<()> {
    if !current_dir()?.join(".git").is_dir() {
        bail!(ERROR_MSG_NOT_PROJECT_ROOT);
    }
    Ok(())
}

pub fn handle_init() -> Result<()> {
    assert_git_repo_root()?;
    let config = Config::default();
    let config_string = to_string(&config)?;
    create_dir_all("ripc/sessions")?;
    write("ripc/config.toml", config_string)?;
    Ok(())
}

fn is_initialized() -> Result<bool> {
    let current_dir = current_dir()?;
    let git_dir = current_dir.join(".git");
    let ripc_dir = current_dir.join("ripc");
    let config_file = ripc_dir.join("config.toml");
    let sessions_dir = ripc_dir.join("sessions");
    let is_initialized =
        git_dir.is_dir() && ripc_dir.is_dir() && config_file.is_file() && sessions_dir.is_dir();
    Ok(is_initialized)
}

pub fn handle_new_card(args: &NewCardArgs) -> Result<()> {
    let curr_dir = PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(&curr_dir);
    if !path.is_dir() {
        bail!("Card path is not a valid directory.")
    }
    let card = Card::default();
    let card_str = to_string(&card)?;
    let ripcard_path = path.join("ripcard.toml");
    write(ripcard_path, card_str)?;
    let question_path = path.join("question.md");
    write(question_path, "# Question\n")?;
    let answer_path = path.join("answer.md");
    write(answer_path, "# Answer\n")?;
    Ok(())
}

pub fn handle_session_start(_args: &SessionMethodArgs) -> Result<()> {
    assert_git_repo_root()?;
    if !is_initialized()? {
        handle_init()?;
    }
    // execute the start routine/sequence
    Ok(())
}

pub fn handle_session_progress(_args: &SessionProgressArgs) -> Result<()> {
    todo!()
}
