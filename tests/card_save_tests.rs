use std::{fs, path::PathBuf};

use anyhow::Result;
use ripcards::{cards::Card, cli::NewCardArgs, handlers::handle_init, utils};
use tempfile::tempdir;

#[test]
fn test_card_save() -> Result<()> {
    let temp_dir = tempdir()?;
    let root = temp_dir.path();
    utils::set_curr_dir(root)?;
    let curr_dir = root;
    handle_init()?;
    let arg = NewCardArgs {
        path: Some(PathBuf::from("some/card/path")),
    };
    let card = Card::new(&root, &curr_dir, &arg)?;
    card.save(root)?;

    let card_files = [
        card.config_file_path_rel(),
        card.question_file_path_rel(),
        card.answer_file_path_rel(),
    ];

    assert!(card_files.iter().all(|p| root.join(p).is_file()));

    let config_content = fs::read_to_string(root.join(&card_files[0]))?;
    let question_content = fs::read_to_string(root.join(&card_files[1]))?;
    let answer_content = fs::read_to_string(root.join(&card_files[2]))?;

    assert!(config_content.contains("[properties]"));
    assert!(config_content.contains("id = "));
    assert!(question_content.contains("# Question"));
    assert!(answer_content.contains("# Answer"));

    Ok(())
}
