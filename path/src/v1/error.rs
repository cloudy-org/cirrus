use std::env;
use cirrus_error::v1::error::CError;

static PLATFORM: &str = env::consts::OS;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    DirNotFoundForPlatform
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::DirNotFoundForPlatform => format!(
                "Failed to search for directory path for your platform ({})! ",
                PLATFORM
            ) + "Report right away! Maybe your platform isn't supported in dirs-rs (https://codeberg.org/dirs/dirs-rs).",
        }.to_string()
    }

    fn actual_error(&self) -> Option<String> {
        // there's no errors at the moment that carry "actual error".
        match self {
            Error::DirNotFoundForPlatform => Some("Dirs-rs method returned none!".to_string()),
        }
    }
}