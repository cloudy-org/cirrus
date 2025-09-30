use cirrus_error::v1::error::CError;

pub type Result<T, E = Error> = std::result::Result<T, E>; 

#[derive(Debug)]
pub enum Error {
    FailedToParseHexCode(String)
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::FailedToParseHexCode(actual_error) => format!("Failed to parse hex code! \n\nError: '{}'", actual_error)
        }
    }

    fn actual_error(&self) -> Option<String> {
        // there's no errors at the moment that carry "actual error".
        match self {
            Error::FailedToParseHexCode(actual_error) => Some(actual_error.into()),
        }
    }
}