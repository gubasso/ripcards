use std::{
    collections::HashSet,
    env::current_dir,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    cli::NewCardArgs,
    methods::CardMethod,
    utils::{create_directory, get_relative_path, write_file_contents},
};

#[derive(PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
struct Tag(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Card {
    id: PathBuf,
    root: PathBuf,
    full_path: PathBuf,
    config_file_rel_path: PathBuf,
    question_file_rel_path: PathBuf,
    answer_file_rel_path: PathBuf,
    method: CardMethod,
    tags: HashSet<Tag>,
}

impl Card {
    pub fn id(&self) -> &Path {
        &self.id
    }

    pub fn new<P: AsRef<Path>>(root: P, curr_dir: P, args: &NewCardArgs) -> Result<Self> {
        let dot_path = PathBuf::from(".");
        let root = root.as_ref().to_path_buf();
        let curr_dir = curr_dir.as_ref().to_path_buf();
        let path_arg_rel = args.path.as_ref().unwrap_or(&dot_path);
        if !root.is_absolute() {
            bail!("Card::new: root path '{}' must be absolute", root.display());
        }
        if !curr_dir.is_absolute() {
            bail!(
                "Card::new: curr_dir path '{}' must be absolute",
                curr_dir.display()
            );
        }
        if !path_arg_rel.is_relative() {
            bail!(
                "Card::new: path parameter arg '{}' must be relative to the current directory",
                path_arg_rel.display()
            );
        }

        let curr_dir_full_path: PathBuf = if path_arg_rel == &dot_path {
            current_dir().context("Card::new(): Failed to get current_dir")?
        } else {
            root.join(path_arg_rel)
        };

        let id = get_relative_path(&root, &curr_dir_full_path)?;

        Ok(Self {
            id: id.clone(),
            root: root.clone(),
            full_path: root.join(&id),
            config_file_rel_path: id.join("ripcard.toml"),
            question_file_rel_path: id.join("question.md"),
            answer_file_rel_path: id.join("answer.md"),
            method: CardMethod::default(),
            tags: HashSet::new(),
        })
    }

    pub fn create_card_files(&self) -> Result<[PathBuf; 3]> {
        create_directory(&self.full_path)?;

        let config_file_content = toml::to_string(&self)?;
        write_file_contents(&self.config_file_rel_path, config_file_content)?;

        let question_file_content = "# Question\n\n".to_string();
        write_file_contents(&self.question_file_rel_path, question_file_content)?;

        let answer_file_content = "# Answer\n\n".to_string();
        write_file_contents(&self.answer_file_rel_path, answer_file_content)?;

        Ok([
            self.config_file_rel_path.clone(),
            self.question_file_rel_path.clone(),
            self.answer_file_rel_path.clone(),
        ])
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use anyhow::Result;

    use crate::{cards::Card, cli::NewCardArgs, methods::CardMethod};

    fn get_handle_new_card_args() -> [NewCardArgs; 4] {
        [
            NewCardArgs { path: None },
            NewCardArgs {
                path: Some(PathBuf::from(".")),
            },
            NewCardArgs {
                path: Some(PathBuf::from("in/a/card/path")),
            },
            NewCardArgs {
                path: Some(PathBuf::from("/absolute/path/as/input")),
            },
        ]
    }

    #[test]
    fn test_card_new_from_root() -> Result<()> {
        let root = PathBuf::from("/root");
        let curr_dir = PathBuf::from("/root");
        let args_arr = get_handle_new_card_args();
        for args in args_arr.iter() {
            let res = Card::new(&root, &curr_dir, args);

            match &args.path {
                None => {
                    assert!(
                        res.is_err(),
                        "test_card_new_from_root: Card::new must return err if \
                            card is created at the project root."
                    );
                }
                Some(path) if path == Path::new(".") => {
                    assert!(
                        res.is_err(),
                        "test_card_new_from_root: Card::new must return err if \
                            card is created at the project root."
                    );
                }
                Some(path) if !path.is_relative() => {
                    assert!(
                        res.is_err(),
                        "test_card_new_from_root: Card::new: path parameter \
                            arg '{}' must be relative to the current directory",
                        path.display()
                    );
                }
                Some(_) => {
                    let card = res?;
                    let id = PathBuf::from("in/a/card/path");
                    assert_eq!(
                        card,
                        Card {
                            id: id.clone(),
                            root: root.clone(),
                            full_path: root.join(&id),
                            config_file_rel_path: id.join("ripcard.toml"),
                            question_file_rel_path: id.join("question.md"),
                            answer_file_rel_path: id.join("answer.md"),
                            method: CardMethod::default(),
                            tags: HashSet::new(),
                        }
                    )
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_card_new_from_sub_path() -> Result<()> {
        let root = PathBuf::from("/root");
        let curr_dir = PathBuf::from("/root/sub/path");
        let args_arr = get_handle_new_card_args();
        for args in args_arr.iter() {
            let card = Card::new(&root, &curr_dir, args)?;

            match &args.path {
                None => {}
                Some(path) if path == Path::new(".") => {}
                Some(_) => {
                    let id_path = PathBuf::from("some/sub/path");
                    assert_eq!(
                        card,
                        Card {
                            id: id_path.clone(),
                            root: root.clone(),
                            full_path: root.join(&id_path),
                            config_file_rel_path: id_path.join("ripcard.toml"),
                            question_file_rel_path: id_path.join("question.md"),
                            answer_file_rel_path: id_path.join("answer.md"),
                            method: CardMethod::default(),
                            tags: HashSet::new(),
                        }
                    )
                }
            }
        }

        Ok(())
    }
}
