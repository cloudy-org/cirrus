use std::{fmt::Display, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    EditorCouldNotOpenFailure { error: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EditorCouldNotOpenFailure { error } => write!(
                f,
                "Failed to open system / user preferred text editor! \n\nError: {error}"
            ),
        }
    }
}