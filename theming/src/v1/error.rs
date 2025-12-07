use cirrus_error::v1::error::CError;

pub type Result<T, E = Error> = std::result::Result<T, E>; 

#[derive(Debug)]
pub enum Error {
    FailedToParseHexCode(String, String),
    FailedToReadThemeToml(String),
    FailedToFindThemeToml(String),
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::FailedToParseHexCode(actual_error, hex_code) => format!(
                "Failed to parse hex code! Hex code: '{}' \n\nError: '{}'",
                hex_code,
                actual_error
            ),
            Error::FailedToReadThemeToml(actual_error) => format!(
                "Failed to read the 'theme.toml' config file, it may be corrupted \
                or the reason below may state otherwise! \n\nError: {}",
                actual_error
            ),
            Error::FailedToFindThemeToml(actual_error) => format!(
                "Failed to find the 'theme.toml' config file! \n\nError: '{}'",
                actual_error
            ),
        }
    }

    fn actual_error(&self) -> Option<String> {
        // there's no errors at the moment that carry "actual error".
        match self {
            Error::FailedToParseHexCode(actual_error, ..) => Some(actual_error.into()),
            Error::FailedToReadThemeToml(actual_error, ..) => Some(actual_error.into()),
            Error::FailedToFindThemeToml(actual_error, ..) => Some(actual_error.into())
        }
    }
}