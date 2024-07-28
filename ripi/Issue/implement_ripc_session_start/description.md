# implement_ripc_session_start (Issue)

- [ ] find cards
  - test_find_cards: separate in multiple tests
    - only matches
    - no matches
  - case where didn't find any
- [ ] for each file match, test if exists a question and answer file
  - if not: print that inconsistency, don't include as a valid card


```rs
use walkdir::WalkDir;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

fn find_dirs_with_ripcard_and_method_leitner(root: &Path) -> Result<Vec<PathBuf>> {
    let mut matching_dirs = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors
        .filter(|e| e.file_type().is_file()) // Include only files
        .filter(|e| e.file_name() == "ripcard.toml") // Include only files named "ripcard.toml"
    {
        let content = fs::read_to_string(entry.path())?;
        if content.contains("[method.leitner]") {
            if let Some(parent_dir) = entry.path().parent() {
                matching_dirs.push(parent_dir.to_path_buf());
            }
        }
    }

    Ok(matching_dirs)
}

fn main() -> Result<()> {
    let root = std::env::current_dir()?;
    let matching_dirs = find_dirs_with_ripcard_and_method_leitner(&root)?;

    if matching_dirs.is_empty() {
        println!("No matching directories found.");
    } else {
        println!("Found matching directories:");
        for dir in matching_dirs {
            println!("{}", dir.display());
        }
    }

    Ok(())
}
```
