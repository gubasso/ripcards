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
    utils::{self, find_ripc_root},
};

#[derive(PartialEq, Eq, Hash, Default, Serialize, Deserialize, Debug)]
struct Tag(String);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Card {
    properties: CardProperties,
    methods: Vec<CardMethod>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct CardProperties {
    id: CardId,
    tags: HashSet<Tag>,
}

impl CardProperties {
    pub fn new(id: CardId) -> Self {
        Self {
            id,
            tags: HashSet::new(),
        }
    }
}

impl Card {
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

        let card_rel_path = utils::get_relative_path(&root, &curr_dir_full_path)?;
        let id = CardId::from(card_rel_path)?;

        Ok(Self {
            properties: CardProperties::new(id),
            methods: vec![CardMethod::default()],
        })
    }

    pub fn id(&self) -> &Path {
        &self.properties.id.0
    }

    fn full_path(&self) -> Result<PathBuf> {
        let root = find_ripc_root()?;
        Ok(root.join(self.id()))
    }

    pub fn config_file_path_abs(&self) -> Result<PathBuf> {
        Ok(self.full_path()?.join("ripcard.toml"))
    }
    pub fn config_file_path_rel(&self) -> PathBuf {
        self.id().join("ripcard.toml")
    }

    pub fn question_file_path_abs(&self) -> Result<PathBuf> {
        Ok(self.full_path()?.join("question.md"))
    }
    pub fn question_file_path_rel(&self) -> PathBuf {
        self.id().join("question.md")
    }

    pub fn answer_file_path_abs(&self) -> Result<PathBuf> {
        Ok(self.full_path()?.join("answer.md"))
    }
    pub fn answer_file_path_rel(&self) -> PathBuf {
        self.id().join("answer.md")
    }

    pub fn save(&self) -> Result<()> {
        utils::create_dir(self.full_path()?)?;
        let config_file_content = toml::to_string(&self)?;
        utils::write_file_contents(self.config_file_path_abs()?, config_file_content)?;
        let question_file_content = "# Question\n\n".to_string();
        utils::write_file_contents(self.question_file_path_abs()?, question_file_content)?;
        let answer_file_content = "# Answer\n\n".to_string();
        utils::write_file_contents(self.answer_file_path_abs()?, answer_file_content)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardId(PathBuf);

impl CardId {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().is_absolute() {
            bail!(
                "CardId::from: path '{}' must be relative, not absolute.",
                path.as_ref().display()
            );
        }
        Ok(CardId(path.as_ref().to_path_buf()))
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
    };

    use anyhow::Result;
    use tempfile::tempdir;

    use crate::{
        cards::{Card, CardId},
        cli::NewCardArgs,
        methods::CardMethod,
        utils::{self, get_handle_new_card_args, get_relative_path},
    };

    use super::CardProperties;

    fn get_default_card(id: CardId) -> Card {
        Card {
            properties: CardProperties::new(id),
            methods: vec![CardMethod::default()],
        }
    }

    #[test]
    fn test_card_new_path_arg_absolute() -> Result<()> {
        let root = Path::new("/root");
        let curr_dir = Path::new("/root/curr_dir");
        let arg = NewCardArgs {
            path: Some(PathBuf::from("/absolute/path/as/input")),
        };
        let res = Card::new(&root, &curr_dir, &arg);
        assert!(
            res.is_err(),
            "test_card_new_path_arg_absolute: Card::new: path parameter \
                arg '{}' must be relative to the current directory, not absolute.",
            arg.path.unwrap().display()
        );
        Ok(())
    }

    #[test]
    fn test_card_new_from_root() -> Result<()> {
        let dot_path = PathBuf::from(".");
        let temp_dir = tempdir()?;
        let root = temp_dir.into_path();
        let curr_dir = root.clone();
        utils::set_curr_dir(&curr_dir)?;
        let args_arr = get_handle_new_card_args();

        for args in args_arr.iter() {
            let res = Card::new(&root, &curr_dir, args);
            let path_args_rel = args.path.as_ref().unwrap_or(&dot_path);
            if path_args_rel == Path::new(".") {
                assert!(
                    res.is_err(),
                    "test_card_new_from_root: Card::new must return err if \
                        card is created at the project root."
                );
            } else {
                let card = res?;
                let card_rel_path = PathBuf::from("in/a/card/path");
                let id = CardId::from(&card_rel_path)?;

                assert_eq!(card, get_default_card(id))
            }
        }

        Ok(())
    }

    #[test]
    fn test_card_new_from_sub_path() -> Result<()> {
        let dot_path = PathBuf::from(".");
        let temp_dir = tempdir()?;
        let root = temp_dir.into_path();
        let curr_dir = root.join("sub/path/cmd");
        utils::set_curr_dir(&curr_dir)?;
        let args_arr = get_handle_new_card_args();

        for args in args_arr.iter() {
            let card = Card::new(&root, &curr_dir, args)?;
            let path_args_rel = args.path.as_ref().unwrap_or(&dot_path);
            let card_rel_path = if path_args_rel == Path::new(".") {
                get_relative_path(&root, &curr_dir)?
            } else {
                PathBuf::from("in/a/card/path")
            };
            let id = CardId::from(&card_rel_path)?;

            assert_eq!(card, get_default_card(id))
        }

        Ok(())
    }
}
