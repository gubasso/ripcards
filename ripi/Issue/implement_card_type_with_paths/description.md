# implement_card_type_with_paths (Issue)

- root_path
- card_id
- full_path

---

- [x] Card::new test all possible scenarios:
  - [x] sub/path
    - . / none -> card_id == some/sub/path
    - some/sub/path -> card_id == sub/path/some/sub/path
  - [x] root -> test_card_new_from_root
    - . / none -> error
    - some/sub/path -> card_id == some/sub/path

handle_new_card possible scenarios:


```
- root ->
  - . / none
  - some/sub/path
- not root
  - . / none
  - some/sub/path
- sub/path
  - . / none
  - some/sub/path
```


test Card::new()

test card.create_card_files

- [x] reimplement get_relative_path, encapsulate error (not option, but error)
- [x] implement create_card_files returning paths of the files created
