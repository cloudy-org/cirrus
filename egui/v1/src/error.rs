// use cirrus_error::error::CError;

// #[deprecated(note="This CError trait is a temporary fix to allow the CError trait
// from the `cirrus_error` crate to be used in `cirrus_egui` crate. Use it for now but this
// is a warning that it will be removed in the future hopefully for a better alternative.")]
// pub trait EguiCError: CError + Clone {}

use cirrus_error::error::CError;

#[derive(Debug)]
pub enum Error {
    FailedToSaveConfig(String),
    FailedToFindPid(String),

    UserConfigPathNotFound { error: String },
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::FailedToSaveConfig(actual_error) => format!("Failed to save config toml file! \n\nError: {}", actual_error),
            Error::FailedToFindPid(actual_error) => format!("Failed to get process id! \n\nError: {}", actual_error),
            Error::UserConfigPathNotFound { error } => format!("Failed to get user config path: {}", error),
        }
    }

    fn actual_error(&self) -> Option<String> {
        match self {
            Error::FailedToSaveConfig(actual_error) => Some(actual_error.into()),
            Error::FailedToFindPid(actual_error) => Some(actual_error.into()),
            Error::UserConfigPathNotFound { error } => Some(error.into()),
        }
    }
}
