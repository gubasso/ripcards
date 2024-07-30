# implement_ripc_session_start (Issue)

- [ ] find cards
  - test_find_cards: separate in multiple tests
    - only matches
    - no matches
  - case where didn't find any
- [ ] for each file match, test if exists a question and answer file
  - if not: print that inconsistency, don't include as a valid card

- [x] new card: will create a path if it doesn't exists


```rs
#[test]
fn test_find_cards_with_matches() -> Result<()> {
    let (temp_dir, curr_dir) = setup_test_directory()?;
    create_test_files(&curr_dir)?;

    let cards = find_cards(&curr_dir, None)?;

    let correct_matches = HashSet::from([
        curr_dir.join("p1/pb"),
        curr_dir.join("p1/pa/px"),
        curr_dir.join("p2"),
        curr_dir.join("p2/pa"),
    ]);

    assert_eq!(cards, correct_matches, "Found cards do not match expected cards");

    // Ensure the TempDir is not dropped prematurely
    drop(temp_dir);

    Ok(())
}

#[test]
fn test_find_cards_no_matches() -> Result<()> {
    let (temp_dir, curr_dir) = setup_test_directory()?;

    // Create only non-matching files
    write_file_contents(curr_dir.join("p1/ripcard.toml"), "lala")?;
    write_file_contents(curr_dir.join("p1/pa/ripcard.toml"), "[method.other]")?;

    let cards = find_cards(&curr_dir, None)?;

    assert!(cards.is_empty(), "Expected no cards to be found");

    drop(temp_dir);

    Ok(())
}

#[test]
fn test_find_cards_empty_directory() -> Result<()> {
    let temp_dir = tempdir()?;
    let curr_dir = temp_dir.path().to_path_buf();

    let cards = find_cards(&curr_dir, None)?;

    assert!(cards.is_empty(), "Expected no cards in an empty directory");

    Ok(())
}
```
