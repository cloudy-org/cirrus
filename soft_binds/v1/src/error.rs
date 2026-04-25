use std::{fmt::Display, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    ParseToEguiKeyFailure { key_string: String },
    UnknownKeyBindChar { char: char, position: u32 }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParseToEguiKeyFailure { key_string } => write!(
                f,
                "Failed to parse the key bind '{key_string}' to an egui key!"
            ),
            Error::UnknownKeyBindChar { char, position } => write!(
                f,
                "Unknown key bind key '{char}' at position ({position})!"
            ),
        }
    }
}