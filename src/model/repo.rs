use crate::model::tool::LATEST_VERSION;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum RepoError {
    /// Either repository or tag is not found due to misconfiguration
    NotFound {
        owner: String,
        repo: String,
        tag: String,
    },
}

impl Display for RepoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::NotFound { owner, repo, tag } => match tag {
                _ if tag == LATEST_VERSION => {
                    write!(f, "The {owner}/{repo} doesn't exist or has no releases.")
                }
                _ => write!(
                    f,
                    "The {owner}/{repo} doesn't exist or {tag} was not found."
                ),
            },
        }
    }
}
