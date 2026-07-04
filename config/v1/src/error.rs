use std::fmt::Display;

use cirrus_error::error::CError;

#[derive(Debug)]
pub enum Error {
    // TODO: update to struct-like enums
    FailedToCreateConfigFile(String),
    FailedToCreateConfigDirectory(String),
    FailedToReadConfig(String),
    FailedToWriteToConfig(String),

    UserConfigPathNotFound { error: String },
    TemplateConfigParseFailure { error: String },
}

impl CError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // TODO: remove actual error in display
            Error::FailedToCreateConfigFile(..) => write!(
                f, "Failed to create config file!"
            ),
            Error::FailedToCreateConfigDirectory(..) => write!(
                f, "Failed to create config directory!"
            ),
            Error::FailedToReadConfig(..) => write!(
                f, "Failed to read config toml file!"
            ),
            Error::FailedToWriteToConfig(..) => write!(
                f, "Failed to write to config toml file!"
            ),
            Error::UserConfigPathNotFound { .. } => write!(
                f, "Failed to get user config path"
            ),
            Error::TemplateConfigParseFailure { .. } => write!(
                f, "Failed to parse template config! Report immediately, this should never be the case!"
            )
        }
    }
}