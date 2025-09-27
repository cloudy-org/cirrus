use cirrus_error::v1::error::CError;

pub enum Error {
    FailedToCreateConfigFile(String),
    FailedToCreateConfigDirectory(String),
    FailedToReadConfig(String),
    FailedToWriteToConfig(String)
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::FailedToCreateConfigFile(actual_error) => format!("Failed to create config file! Error: {}", actual_error),
            Error::FailedToCreateConfigDirectory(actual_error) => format!("Failed to create config directory! Error: {}", actual_error),
            Error::FailedToReadConfig(actual_error) => format!("Failed to read config toml file! Error: {}", actual_error),
            Error::FailedToWriteToConfig(actual_error) => format!("Failed to write to config toml file! Error: {}", actual_error),
        }.to_string()
    }

    fn actual_error(&self) -> Option<String> {
        match self {
            Error::FailedToCreateConfigFile(actual_error) => Some(actual_error.into()),
            Error::FailedToCreateConfigDirectory(actual_error) => Some(actual_error.into()),
            Error::FailedToReadConfig(actual_error) => Some(actual_error.into()),
            Error::FailedToWriteToConfig(actual_error) => Some(actual_error.into()),
        }
    }
}