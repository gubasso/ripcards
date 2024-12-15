use std::path::Path;

use anyhow::{Context, Result};
use cmd_lib::run_fun;
use ripcards::{
    handlers::{handle_init, handle_new_card},
    msgs::git_commit_msg_ripc_new,
    utils::{self, get_handle_new_card_args},
};
use tempfile::tempdir;

fn assert_new_card_files_and_git(card_dir: &Path) -> Result<()> {
    assert!(
        card_dir.is_dir(),
        "assert_new_card_files_and_git: Card dir '{}' must exists.",
        card_dir.display()
    );
    let out_files_not_added = run_fun!(git status --porcelain)?;
    let out_git_log = run_fun!(git log --oneline)?;

    let new_card_files = ["ripcard.toml", "question.md", "answer.md"];

    for file in new_card_files.into_iter() {
        let path = card_dir.join(file);
        let path_str = path.to_str().unwrap();
        assert!(
            path.is_file(),
            "assert_new_card_files_and_git: {} file must exist",
            path_str
        );
        assert!(!out_files_not_added.contains(path_str));
        let card_id = card_dir.to_str().unwrap();
        assert!(out_git_log.contains(&git_commit_msg_ripc_new(card_id)));
    }

    Ok(())
}

#[test]
fn test_handle_new_card_at_root() -> Result<()> {
    let temp_dir = tempdir()?;
    let root = temp_dir.into_path();
    utils::set_curr_dir(&root)?;
    utils::create_dir(".git")?;
    handle_init()?;

    let args_arr = get_handle_new_card_args();

    for args in args_arr.iter() {
        let res = handle_new_card(args);
        let out = run_fun!(git log --oneline)?;
        match &args.path {
            None => {
                assert!(
                    res.is_err(),
                    "Must return err if card is created at the project root."
                );
            }
            Some(path) if path == Path::new(".") => {
                assert!(
                    res.is_err(),
                    "Must return err if card is created at the project root."
                );
            }
            Some(path) => {
                res.with_context(|| format!(
                    "test_handle_new_card_at_root: must create the card correctly at the path: '{}'",
                    path.display()
                ))?;
                let new_card_path = root.join(path);
                assert_new_card_files_and_git(&new_card_path)?;
            }
        }
    }

    Ok(())
}
