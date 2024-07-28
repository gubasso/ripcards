use std::{
    env::current_dir,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use anyhow::Result;
use toml::to_string;

use crate::{cards::Card, cli::NewCardArgs, config::create_config_file, context::Context};

pub fn handle_init() -> Result<()> {
    let _ctx = Context::new(current_dir()?);
    create_dir_all("ripc/sessions")?;
    create_config_file()?;
    Ok(())
}

pub fn handle_new_card(args: &NewCardArgs) -> Result<()> {
    let curr_dir = PathBuf::from(".");
    let path = args.path.as_ref().unwrap_or(&curr_dir);
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
