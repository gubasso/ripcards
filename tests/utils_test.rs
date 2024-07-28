use std::{
    collections::HashSet,
    env::set_current_dir,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use anyhow::Result;
use ripcards::utils::find_cards;
use tempfile::tempdir;

#[test]
fn test_find_cards() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    let curr_dir = temp_dir.path();
    create_dir_all("p1/pa/px")?;
    create_dir_all("p1/pb/pz")?;
    create_dir_all("p2/pa/pt")?;
    // no match
    write("p1/ripcard.toml", "lala")?;
    write("p1/pa/ripcard.toml", "[method.other]")?;
    // matches
    write("p1/pb/ripcard.toml", "[method.leitner]")?;
    write("p1/pa/px/ripcard.toml", "[method.leitner]")?;
    write("p2/ripcard.toml", "[method.leitner]")?;
    write("p2/pa/ripcard.toml", "[method.leitner]")?;
    let cards = find_cards(curr_dir, None)?;
    let correct_matches = HashSet::from([
        curr_dir.join("p1/pb").to_path_buf(),
        curr_dir.join("p1/pa/px").to_path_buf(),
        curr_dir.join("p2").to_path_buf(),
        curr_dir.join("p2/pa").to_path_buf(),
    ]);
    assert_eq!(cards, correct_matches);
    Ok(())
}
