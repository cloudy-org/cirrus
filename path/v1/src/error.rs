use std::{env, fmt::Display};

static PLATFORM: &str = env::consts::OS;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    PathNotFoundForPlatform,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::PathNotFoundForPlatform => write!(
                f,
                "This path is not supported for your platform yet ({PLATFORM}) in 'cirrus_path'! \
                    Feel free to add support or report it here: https://github.com/cloudy-org/cirrus",
            ),
        }
    }
}