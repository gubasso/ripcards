use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::utils;

pub fn find_cards_paths(root: &Path) -> HashSet<PathBuf> {
    let mut matching_dirs = HashSet::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name() == "ripcard.toml")
    {
        if let Some(parent_dir) = entry.path().parent() {
            let rel_path = utils::get_relative_path(root, parent_dir).unwrap();
            matching_dirs.insert(rel_path);
        }
    }
    matching_dirs
}

#[cfg(test)]
mod test {
    use std::{collections::HashSet, fs::File, path::PathBuf};

    use anyhow::Result;
    use tempfile::tempdir;

    use crate::utils;

    use super::find_cards_paths;

    #[test]
    fn test_find_cards_empty_directory() -> Result<()> {
        let root = tempdir()?;
        let cards_paths = find_cards_paths(root.as_ref());
        assert!(cards_paths.is_empty());
        Ok(())
    }

    #[test]
    fn test_find_cards_no_matching_files() -> Result<()> {
        let root = tempdir()?;
        File::create(root.path().join("some_random_file"))?;
        let cards_paths = find_cards_paths(root.as_ref());
        assert!(cards_paths.is_empty());
        Ok(())
    }

    #[test]
    fn test_find_cards_matching_single_file() -> Result<()> {
        let root = tempdir()?;
        let card_dir = PathBuf::from("sub/dir");
        let config_file = PathBuf::from("ripcard.toml");
        utils::write_file_contents(root.path().join(&card_dir).join(config_file), "")?;
        let cards_paths = find_cards_paths(root.as_ref());
        let expected = HashSet::from([card_dir]);
        assert_eq!(cards_paths, expected);
        Ok(())
    }

    #[test]
    fn test_find_cards_matching_multiple_file() -> Result<()> {
        let root = tempdir()?;
        let dir1 = PathBuf::from("dir1");
        let dir2 = PathBuf::from("dir2");
        let config_file = PathBuf::from("ripcard.toml");
        utils::write_file_contents(root.path().join(&dir1).join(&config_file), "")?;
        utils::write_file_contents(root.path().join(&dir2).join(config_file), "")?;
        let cards_paths = find_cards_paths(root.as_ref());
        let expected = HashSet::from([dir1, dir2]);
        assert_eq!(cards_paths, expected);
        Ok(())
    }

    #[test]
    fn test_find_cards_nested() -> Result<()> {
        let root = tempdir()?;
        let dir = PathBuf::from("dir");
        let subdir = dir.join("subdir");
        let config_file = PathBuf::from("ripcard.toml");
        utils::write_file_contents(root.path().join(&dir).join(&config_file), "")?;
        utils::write_file_contents(root.path().join(&subdir).join(&config_file), "")?;
        let cards_paths = find_cards_paths(root.as_ref());
        let expected = HashSet::from([dir, subdir]);
        assert_eq!(cards_paths, expected);
        Ok(())
    }

    #[test]
    fn test_find_cards_ignore_other_files() -> Result<()> {
        let root = tempdir()?;
        let dir = PathBuf::from("dir");
        let config_file = "ripcard.toml";
        let other_file = "other_file.txt";
        utils::write_file_contents(root.path().join(&dir).join(config_file), "")?;
        utils::write_file_contents(root.path().join(&dir).join(other_file), "")?;
        utils::write_file_contents(root.path().join(&dir).join("subdir").join(other_file), "")?;
        let cards_paths = find_cards_paths(root.as_ref());
        let expected = HashSet::from([dir]);
        assert_eq!(cards_paths, expected);
        Ok(())
    }

    #[test]
    fn test_find_cards_ignore_directories_named_ripcard_toml() -> Result<()> {
        let root = tempdir()?;
        let dir = root.as_ref().join("ripcard.toml");
        utils::create_dir(dir)?;
        let cards_paths = find_cards_paths(root.as_ref());
        assert!(cards_paths.is_empty());
        Ok(())
    }
}
