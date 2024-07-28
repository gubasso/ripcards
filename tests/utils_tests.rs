mod common;

use std::{
    collections::HashSet,
    env::set_current_dir,
    fs::{create_dir, create_dir_all, write, File},
};

use common::setup_temp_dir_handle_init;

use anyhow::Result;
use ripcards::{
    handlers::handle_init,
    utils::{find_cards, find_ripc_root, is_ripc_root},
};
use tempfile::tempdir;

fn create_mock_dirs() -> Result<()> {
    create_dir_all("p1/pa/px")?;
    create_dir_all("p1/pb/pz")?;
    create_dir_all("p2/pa/pt")?;
    Ok(())
}

#[test]
fn test_is_ripc_root() -> Result<()> {
    let temp_dir = tempdir()?;
    set_current_dir(&temp_dir)?;
    assert!(!is_ripc_root(temp_dir.path()));
    create_dir(".git")?;
    assert!(!is_ripc_root(temp_dir.path()));
    create_dir_all("ripc/sessions")?;
    assert!(!is_ripc_root(temp_dir.path()));
    File::create("ripc/config.toml")?;
    assert!(is_ripc_root(temp_dir.path()));
    Ok(())
}

#[test]
fn test_is_ripc_root_with_handle_init() -> Result<()> {
    let ripc_root = setup_temp_dir_handle_init()?;
    assert!(is_ripc_root(&ripc_root));
    Ok(())
}

#[test]
fn test_find_ripc_root() -> Result<()> {
    let temp_dir = tempdir()?.into_path();
    let some_random_subdir = temp_dir.join("some/random/dir");
    create_dir_all(&some_random_subdir)?;

    set_current_dir(&temp_dir)?;
    let res = find_ripc_root();
    assert!(res.is_err(), "There is no RipCards root to be found.");

    set_current_dir(&some_random_subdir)?;
    let res = find_ripc_root();
    assert!(res.is_err(), "There is no RipCards root to be found.");

    set_current_dir(&temp_dir)?;
    handle_init()?;

    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    set_current_dir(&some_random_subdir)?;
    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    Ok(())
}

// #[test]
// fn test_find_ripc_root_at_subdir() -> Result<()> {
//     let temp_dir = tempdir()?;
//     set_current_dir(&temp_dir)?;
//     handle_init()?;
//
//
//
//
//     // let res = find_ripc_root();
//     // assert!(res.is_err(), "There is no RipCards root");
//     // let ripc_root = find_ripc_root()?;
//     // assert_eq!(ripc_root, temp_dir.into_path());
//     Ok(())
// }

#[test]
fn test_find_cards() -> Result<()> {
    let proj_root = setup_temp_dir_handle_init()?;
    create_mock_dirs()?;
    // no match
    write("p1/ripcard.toml", "lala")?;
    write("p1/pa/ripcard.toml", "[method.other]")?;
    // matches
    write("p1/pb/ripcard.toml", "[method.leitner]")?;
    write("p1/pa/px/ripcard.toml", "[method.leitner]")?;
    write("p2/ripcard.toml", "[method.leitner]")?;
    write("p2/pa/ripcard.toml", "[method.leitner]")?;
    let cards = find_cards(&proj_root, None)?;
    let correct_matches = HashSet::from([
        proj_root.join("p1/pb").to_path_buf(),
        proj_root.join("p1/pa/px").to_path_buf(),
        proj_root.join("p2").to_path_buf(),
        proj_root.join("p2/pa").to_path_buf(),
    ]);
    assert_eq!(cards, correct_matches);
    Ok(())
}
