use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TagError {
    /// Tag is not in the available tags
    NotFound(String, Option<String>),
}

impl Display for Tag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.name)
    }
}

impl Display for TagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(tag, possible_tag) => match possible_tag {
                Some(closest_tag) => write!(
                    f,
                    "There's no tag '{}'. Perhaps you meant '{}'?",
                    tag, closest_tag
                ),
                None => write!(f, "There's no tag '{}'", tag),
            },
        }
    }
}

impl Error for TagError {}
