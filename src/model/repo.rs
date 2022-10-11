use crate::model::tool::ToolInfoTag;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum RepoError {
    /// Either repository or tag is not found due to misconfiguration
    NotFound {
        owner: String,
        repo: String,
        tag: ToolInfoTag,
    },
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::NotFound { owner, repo, tag } => match tag {
                ToolInfoTag::Latest => {
                    write!(f, "The {owner}/{repo} doesn't exist or has no releases.")
                }
                _ => write!(
                    f,
                    "The {owner}/{repo} doesn't exist or {tag} was not found.",
                    tag = tag.to_str_version()
                ),
            },
        }
    }
}
