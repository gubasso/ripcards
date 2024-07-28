use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use anyhow::{bail, Result};
use toml::to_string;

use crate::{
    cards::Card,
    cli::{NewCardArgs, SessionMethodArgs, SessionProgressArgs},
    config::Config,
    utils::{assert_git_repo_root, is_repo_initialized},
};

pub fn handle_init() -> Result<()> {
    assert_git_repo_root()?;
    let config = Config::default();
    let config_string = to_string(&config)?;
    create_dir_all("ripc/sessions")?;
    write("ripc/config.toml", config_string)?;
    Ok(())
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
    if !is_repo_initialized()? {
        handle_init()?;
    }
    // execute the start routine/sequence
    Ok(())
}

pub fn handle_session_progress(_args: &SessionProgressArgs) -> Result<()> {
    todo!()
}
