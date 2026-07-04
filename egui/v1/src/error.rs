// use cirrus_error::error::CError;

// #[deprecated(note="This CError trait is a temporary fix to allow the CError trait 
// from the `cirrus_error` crate to be used in `cirrus_egui` crate. Use it for now but this 
// is a warning that it will be removed in the future hopefully for a better alternative.")]
// pub trait EguiCError: CError + Clone {}

use std::fmt::Display;

use cirrus_error::error::CError;

#[derive(Debug)]
pub enum Error {
    SaveConfigFailure{ error: String },

    UserConfigPathNotFound { error: String },
}

impl CError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SaveConfigFailure { .. } => write!(
                f, "Failed to save config toml file!"
            ),
            Error::UserConfigPathNotFound { .. } => write!(
                f, "Failed to get user config path"
            ),
        }
    }
}