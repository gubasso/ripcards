use std::env::current_dir;

use anyhow::Result;
use cmd_lib::run_cmd;

use crate::{
    cards::Card,
    cli::{NewCardArgs, SessionMethodArgs, SessionProgressArgs},
    config::Config,
    msgs::{git_commit_msg_ripc_new, GIT_COMMIT_MSG_RIPC_INIT},
    utils::{find_ripc_root, git_add_files, write_file_contents},
};

pub fn handle_init() -> Result<()> {
    run_cmd!(git init)?;
    let config = Config::default();
    let config_content = toml::to_string(&config)?;
    let session_dir_keep = "ripc/sessions/.gitkeep";
    let config_file = "ripc/config.toml";
    write_file_contents(session_dir_keep, "")?;
    write_file_contents(config_file, config_content)?;
    run_cmd!(git add "$session_dir_keep" "$config_file")?;
    run_cmd!(git commit -m "$GIT_COMMIT_MSG_RIPC_INIT")?;
    Ok(())
}

pub fn handle_new_card(args: &NewCardArgs) -> Result<()> {
    let root = find_ripc_root()?;
    let curr_dir = current_dir()?;
    let card = Card::new(&root, &curr_dir, args)?;
    card.save(&root)?;
    let files_path_rel = [
        card.config_file_path_rel(),
        card.question_file_path_rel(),
        card.answer_file_path_rel(),
    ];
    git_add_files(&files_path_rel)?;
    let git_commit_msg = git_commit_msg_ripc_new(card.id().to_str().unwrap());
    run_cmd!(git commit -m "$git_commit_msg")?;
    Ok(())
}

pub fn handle_session_start(_args: &SessionMethodArgs) -> Result<()> {
    todo!()
}

pub fn handle_session_progress(_args: &SessionProgressArgs) -> Result<()> {
    todo!()
}
