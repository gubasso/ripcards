use std::{env::current_dir, fs::File, path::Path};

use anyhow::Result;
use ripcards::{
    handlers::handle_init,
    utils::{self, find_ripc_root, is_ripc_root},
};
use tempfile::tempdir;

#[test]
fn test_get_relative_path() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new("some/sub/dir");
    let full_path = base.join(some_sub_dir);
    let relative_path = utils::get_relative_path(&base, &full_path).unwrap();
    assert_eq!(relative_path, some_sub_dir);
    Ok(())
}

#[test]
fn test_get_relative_path_same_path() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new(".");
    let full_path = base.join(some_sub_dir);
    let relative_path = utils::get_relative_path(&base, &full_path);
    assert!(
        relative_path.is_err(),
        "If base and full path are the same, relative path is Error."
    );
    Ok(())
}

#[test]
fn test_get_relative_path_base_bigger_than_full() -> Result<()> {
    let base = current_dir()?;
    let some_sub_dir = Path::new("some/sub/dir");
    let full_path = base.join(some_sub_dir);
    let relative_path = utils::get_relative_path(&full_path, &base);
    assert!(
        relative_path.is_err(),
        "Error if base is a sub dir of full path."
    );
    Ok(())
}

#[test]
fn test_get_relative_path_different_trees() -> Result<()> {
    let base = Path::new("one/dir/path");
    let another_dir = Path::new("another/dir/path");
    let relative_path = utils::get_relative_path(base, another_dir);
    assert!(
        relative_path.is_err(),
        "Error if dirs are from a separate tree (not common ancestor)."
    );
    Ok(())
}

#[test]
fn test_is_ripc_root() -> Result<()> {
    let temp_dir = tempdir()?;
    utils::set_curr_dir(&temp_dir)?;
    assert!(!is_ripc_root(temp_dir.path()));
    utils::create_dir(".git")?;
    assert!(!is_ripc_root(temp_dir.path()));
    utils::create_dir("ripc/sessions")?;
    assert!(!is_ripc_root(temp_dir.path()));
    File::create("ripc/config.toml")?;
    assert!(is_ripc_root(temp_dir.path()));
    Ok(())
}

#[test]
fn test_is_ripc_root_with_handle_init() -> Result<()> {
    let temp_dir = tempdir()?;
    let root = temp_dir.into_path();
    utils::set_curr_dir(&root)?;
    handle_init()?;
    assert!(is_ripc_root(&root));
    Ok(())
}

#[test]
fn test_find_ripc_root() -> Result<()> {
    let temp_dir = tempdir()?.into_path();
    let some_random_subdir = temp_dir.join("some/random/dir");
    utils::create_dir(&some_random_subdir)?;

    utils::set_curr_dir(&temp_dir)?;
    let res = find_ripc_root();
    assert!(
        res.is_err(),
        "The RipCards proj root not found. Repository must be initialized."
    );

    utils::set_curr_dir(&some_random_subdir)?;
    let res = find_ripc_root();
    assert!(
        res.is_err(),
        "The RipCards proj root not found. Repository must be initialized."
    );

    utils::set_curr_dir(&temp_dir)?;
    handle_init()?;

    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    utils::set_curr_dir(&some_random_subdir)?;
    let ripc_root = find_ripc_root()?;
    assert_eq!(ripc_root, temp_dir);

    Ok(())
}
