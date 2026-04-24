use std::{fmt::Display, path::PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>; 

#[derive(Debug)]
pub enum Error {
    ThemeTomlUnsupported { version: String },
    ThemeTomlParseFailure { error: String },
    ThemeTomlNoVersionKey { theme_code_name: String, theme_toml_path: PathBuf },

    PathNotATheme { path: PathBuf },
    ReadThemesPathFailure { themes_path: PathBuf, error: String },

    GlobalConfigParseFailure { error: String },
    HexCodeParseFailure { error: String, hex_string: String },

    FailedToFindThemeToml { error: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HexCodeParseFailure { error, hex_string }  => write!(
                f, "Failed to parse hex code! Hex code: '{hex_string}' \n\nError: '{error}'",
            ),
            Error::ThemeTomlUnsupported { version } => write!(
                f,
                "Unsupported theme version! This current version of the toolkit does not \
                support version '{version}' of theme.toml! Make sure this application is update to date."
            ),
            Error::ThemeTomlParseFailure { error } => write!(
                f, "Failed to read the 'theme.toml' config file, it may be corrupted \
                    or the reason below may state otherwise! \n\nError: {error}",
            ),
            Error::ThemeTomlNoVersionKey { theme_code_name, theme_toml_path } => write!(
                f,
                "Failed to parse the '{theme_code_name}' theme, no 'version' key found in '{}'!",
                theme_toml_path.to_string_lossy().to_string()
            ),
            Error::PathNotATheme { path } => write!(
                f,
                "The path at '{}' was not a theme!",
                path.display()
            ),
            Error::ReadThemesPathFailure { themes_path, error } => write!(
                f,
                "Failed to read themes path '{}'! \n\nError: {error}",
                themes_path.to_string_lossy()
            ),
            Error::GlobalConfigParseFailure { error } => write!(
                f, "Failed to read the global cloudy config file ('~/.config/cloudy/config.toml'), \
                it may be corrupted or the reason below may state otherwise! \n\nError: {error}",
            ),
            Error::FailedToFindThemeToml { error } => write!(
                f, "Failed to find the 'theme.toml' config file! \n\nError: '{error}'",
            ),
        }
    }
}