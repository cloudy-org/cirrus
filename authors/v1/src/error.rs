use std::{fmt::Display, result::Result as StdResult};

use cirrus_git_tag::error::Error as GitTagError;

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    NoAuthorFound,
    IncorrectSyntax { troubled_line: String, error: String },

    GitTag(GitTagError),
}

impl From<GitTagError> for Error {
    fn from(value: GitTagError) -> Self {
        Error::GitTag(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoAuthorFound => write!(
                f, "There needs to be at least one line in \
                the 'AUTHORS.txt' file to represent an author."
            ),
            Error::IncorrectSyntax { troubled_line, error } => write!(
                f, "Incorrect syntax in '{troubled_line}'! \n\nError: {error}"
            ),
            Error::GitTag(error) => write!(
                f, "Failed to parse git tag! \n\nError: {error}"
            )
        }
    }
}