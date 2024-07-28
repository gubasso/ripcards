use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::methods::MethodCard;

#[derive(PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
struct Tag(String);

#[derive(Serialize, Deserialize)]
struct Content {
    question_file: PathBuf,
    answer_file: PathBuf,
    tags: HashSet<Tag>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            question_file: PathBuf::from("question.md"),
            answer_file: PathBuf::from("answer.md"),
            tags: HashSet::default(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Card {
    content: Content,
    method: MethodCard,
}
