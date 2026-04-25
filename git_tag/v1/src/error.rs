use std::{fmt::Display, result::Result as StdResult};

pub type Result<T, E = Error> = StdResult<T, E>;

#[derive(Debug)]
pub enum Error {
    NoAtSymbol { troubled_string: String },
    UnknownPlatform { platform: String },
    IllegalPrefix { prefix_string: String, reason: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoAtSymbol { troubled_string } => write!(
                f,
                "Failed to parse git tag! No '@' symbol to separate prefix \
                    and platform tag was found. What we got: '{}'.",
                troubled_string
            ),
            Error::UnknownPlatform { platform } => write!(
                f, "Failed to parse platform! Platform '{platform}' is unknown!",
            ),
            Error::IllegalPrefix { prefix_string, reason } => write!(
                f, "The git tag prefix '{prefix_string}' is formatted illegally! Reason: {reason}",
            )
        }
    }
}