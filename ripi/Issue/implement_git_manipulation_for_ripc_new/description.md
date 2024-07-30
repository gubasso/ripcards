# implement_git_manipulation_for_ripc_new (Issue)

- each file manipulation must be in functions that will call git
  - ripc init is integrated with git?
  - ripc new is integrated with git?

---


- [ ] ripc new test cases
  - at root, input:
    - none
    - .
    - some/sub/path
  - at some/sub/path, input
    - none
    - .
    - some/sub/path
  - at uninitialized project:
    - none, ., some/sub/path



- review handle_new_card
  - identify the relative_path from the root as the card_id

- [x] git_add_files
- [x] write -> write_file_content
- [x] create_dir_all -> create_directory
- [x] test get_relative_path
