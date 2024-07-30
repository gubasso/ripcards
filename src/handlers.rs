use std::{env::current_dir, fs::File, path::PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cmd_lib::run_cmd;

use crate::{
    cards::Card,
    cli::{NewCardArgs, SessionMethodArgs, SessionProgressArgs},
    config::Config,
    msgs::{git_commit_msg_ripc_new, GIT_COMMIT_MSG_RIPC_INIT},
    utils::{
        create_directory, find_ripc_root, get_relative_path, git_add_files, write_file_contents,
    },
};

pub fn handle_init() -> Result<()> {
    run_cmd!(git init)?;
    let config = Config::default();
    let config_content = toml::to_string(&config)?;
    let session_dir = "ripc/sessions";
    let session_dir_keep = "ripc/sessions/.gitkeep";
    let config_file = "ripc/config.toml";
    create_directory(session_dir)?;
    File::create(session_dir_keep)?;
    write_file_contents(config_file, config_content)?;
    run_cmd!(git add "$session_dir_keep" "$config_file")?;
    run_cmd!(git commit -m "$GIT_COMMIT_MSG_RIPC_INIT")?;
    Ok(())
}

pub fn handle_new_card(args: &NewCardArgs) -> Result<()> {
    let curr_dir_abs_path = current_dir().context("handle_new_card: Failed to get current_dir")?;
    let dot_dir = PathBuf::from(".");
    let new_card_rel_path_arg = args.path.as_ref().unwrap_or(&dot_dir);
    let root_path = find_ripc_root()?;
    let new_card_abs_path = if new_card_rel_path_arg == &dot_dir {
        curr_dir_abs_path
    } else {
        root_path.join(new_card_rel_path_arg)
    };

    if root_path == new_card_abs_path {
        bail!(
            "Can not create a new card at root of RipCards project. \
            A card must be a subdirectory inside the RipCards root directory."
        );
    }

    create_directory(&new_card_abs_path)?;
    let card = Card::default();
    let card_str = toml::to_string(&card)?;
    let card_id = get_relative_path(&root_path, &new_card_abs_path)
        .ok_or_else(|| anyhow!("Card id must be a relative path from the project root."))?;

    let new_card_files_content = [
        ("ripcard.toml", card_str),
        ("question.md", "# Question\n\n".to_string()),
        ("answer.md", "# Answer\n\n".to_string()),
    ];

    let mut git_add_vec = vec![];
    for (fname, content) in new_card_files_content {
        let file_path = card_id.join(fname);
        let file_path_str = file_path.to_str().unwrap();
        write_file_contents(&file_path, &content)?;
        git_add_vec.push(file_path_str.to_string());
    }

    git_add_files(&git_add_vec)?;

    let git_commit_msg = git_commit_msg_ripc_new(card_id.to_str().unwrap());
    run_cmd!(git commit -m "$git_commit_msg")?;

    Ok(())
}

pub fn handle_session_start(_args: &SessionMethodArgs) -> Result<()> {
    todo!()
}

pub fn handle_session_progress(_args: &SessionProgressArgs) -> Result<()> {
    todo!()
}
