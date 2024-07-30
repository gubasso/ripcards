mod common;

use std::{
    collections::HashSet,
    env::current_dir,
    fs::{create_dir, write, File},
    path::Path,
};

use common::setup_temp_dir_handle_init;

use anyhow::Result;
use ripcards::{
    handlers::handle_init,
    utils::{
        create_directory, find_cards, find_ripc_root, get_relative_path, is_ripc_root, set_curr_dir,
    },
};
use tempfile::tempdir;

fn create_mock_dirs() -> Result<()> {
    create_directory("p1/pa/px")?;
    create_directory("p1/pb/pz")?;
    create_directory("p2/pa/pt")?;
    Ok(())
}

#[test]
fn test_get_relative_path() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new("some/sub/dir");
    let full_path = base.join(some_sub_dir);
    let relative_path = get_relative_path(&base, &full_path).unwrap();
    assert_eq!(relative_path, some_sub_dir);
    Ok(())
}

#[test]
fn test_get_relative_path_same_path() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new(".");
    let full_path = base.join(some_sub_dir);
    let relative_path = get_relative_path(&base, &full_path);
    assert!(
        relative_path.is_none(),
        "If base and full path are the same, relative path is None."
    );
    Ok(())
}

#[test]
fn test_get_relative_path_base_bigger_than_full() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new("some/sub/dir");
    let full_path = base.join(some_sub_dir);
    let relative_path = get_relative_path(&full_path, &base);
    assert!(
        relative_path.is_none(),
        "None if base is a sub dir of full path."
    );
    Ok(())
}

#[test]
fn test_get_relative_path_different_trees() -> Result<()> {
    let base = Path::new("one/dir/path");
    let another_dir = Path::new("another/dir/path");
    let relative_path = get_relative_path(base, another_dir);
    assert!(
        relative_path.is_none(),
        "None if dirs are from a separate tree (not common ancestor)."
    );
    Ok(())
}

#[test]
fn test_is_ripc_root() -> Result<()> {
    let temp_dir = tempdir()?;
    set_curr_dir(&temp_dir)?;
    assert!(!is_ripc_root(temp_dir.path()));
    create_dir(".git")?;
    assert!(!is_ripc_root(temp_dir.path()));
    create_directory("ripc/sessions")?;
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
    create_directory(&some_random_subdir)?;

    set_curr_dir(&temp_dir)?;
    let res = find_ripc_root();
    assert!(
        res.is_err(),
        "The RipCards proj root not found. Repository must be initialized."
    );

    set_curr_dir(&some_random_subdir)?;
    let res = find_ripc_root();
    assert!(
        res.is_err(),
        "The RipCards proj root not found. Repository must be initialized."
    );

    set_curr_dir(&temp_dir)?;
    handle_init()?;

    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    set_curr_dir(&some_random_subdir)?;
    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    Ok(())
}

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
