use cirrus_error::v1::error::CError;

#[derive(Debug)]
pub enum Error {
    FailedToCreateConfigFile(String),
    FailedToCreateConfigDirectory(String),
    FailedToReadConfig(String),
    FailedToWriteToConfig(String),

    UserConfigPathNotFound { error: String },
    TemplateConfigParseFailure { error: String },
}

impl CError for Error {
    fn human_message(&self) -> String {
        match self {
            Error::FailedToCreateConfigFile(actual_error) => format!("Failed to create config file!\n\n Error: {}", actual_error),
            Error::FailedToCreateConfigDirectory(actual_error) => format!("Failed to create config directory!\n\n Error: {}", actual_error),
            Error::FailedToReadConfig(actual_error) => format!("Failed to read config toml file!\n\n Error: {}", actual_error),
            Error::FailedToWriteToConfig(actual_error) => format!("Failed to write to config toml file!\n\n Error: {}", actual_error),

            Error::UserConfigPathNotFound { error } => format!("Failed to get user config path: {}", error),
            Error::TemplateConfigParseFailure { .. } => format!(
                "Failed to parse template config! Report immediately, this should never be the case!",
            )
        }.to_string()
    }

    fn actual_error(&self) -> Option<String> {
        match self {
            Error::FailedToCreateConfigFile(actual_error) => Some(actual_error.into()),
            Error::FailedToCreateConfigDirectory(actual_error) => Some(actual_error.into()),
            Error::FailedToReadConfig(actual_error) => Some(actual_error.into()),
            Error::FailedToWriteToConfig(actual_error) => Some(actual_error.into()),

            Error::UserConfigPathNotFound { error } => Some(error.into()),
            Error::TemplateConfigParseFailure { error } => Some(error.into()),
        }
    }
}