use std::{
    env::set_current_dir,
    fs::{create_dir_all, write, File},
    path::PathBuf,
};

use anyhow::{bail, Result};
use cmd_lib::run_cmd;

use crate::{
    cards::Card,
    cli::{NewCardArgs, SessionMethodArgs, SessionProgressArgs},
    config::Config,
    msgs::GIT_COMMIT_MSG_RIPC_INIT,
    utils::{assert_git_repo_root, find_ripc_root, is_repo_initialized},
};

pub fn handle_init() -> Result<()> {
    run_cmd!(git init)?;
    let config = Config::default();
    let config_content = toml::to_string(&config)?;
    let session_dir = "ripc/sessions";
    let session_dir_keep = "ripc/sessions/.gitkeep";
    let config_file = "ripc/config.toml";
    create_dir_all(session_dir)?;
    File::create(session_dir_keep)?;
    write(config_file, config_content)?;
    run_cmd!(git add "$session_dir_keep" "$config_file")?;
    run_cmd!(git commit -m "$GIT_COMMIT_MSG_RIPC_INIT")?;
    Ok(())
}

pub fn handle_new_card(args: &NewCardArgs) -> Result<()> {
    let curr_dir = PathBuf::from(".");
    let new_card_rel_path = args.path.as_ref().unwrap_or(&curr_dir);
    let root_path = find_ripc_root()?;
    let new_card_path = &root_path.join(new_card_rel_path);
    set_current_dir(&root_path)?;
    if &root_path == new_card_path {
        bail!(
            "Can not create a new card at root of RipCards project. \
            A card must be a subdirectory inside the RipCards root directory."
        );
    }
    create_dir_all(new_card_path)?;
    let card = Card::default();
    let card_str = toml::to_string(&card)?;
    write(new_card_path.join("ripcard.toml"), card_str)?;
    write(new_card_path.join("question.md"), "# Question\n\n")?;
    write(new_card_path.join("answer.md"), "# Answer\n\n")?;
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
