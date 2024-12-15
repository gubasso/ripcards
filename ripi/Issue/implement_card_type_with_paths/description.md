# implement_card_type_with_paths (Issue)

- root_path
- card_id
- full_path

---

- [ ] card.save test

- [x] Card::new test all possible scenarios:
  - [x] sub/path
    - . / none -> card_id == some/sub/path
    - some/sub/path -> card_id == sub/path/some/sub/path
  - [x] root -> test_card_new_from_root
    - . / none -> error
    - some/sub/path -> card_id == some/sub/path

- [x] Card::save testing
- [x] Card::save method

- [ ] handle_new_card test all possible scenarios:
  - [ ] root
    - . / none -> error
    - some/sub/path -> card_id == some/sub/path
  - [ ] sub/path
    - . / none -> card_id == some/sub/path
    - some/sub/path -> card_id == sub/path/some/sub/path


test card.create_card_files

- [x] review all tests
- [x] test_find_cards
- [x] reimplement get_relative_path, encapsulate error (not option, but error)
- [x] implement create_card_files returning paths of the files created
