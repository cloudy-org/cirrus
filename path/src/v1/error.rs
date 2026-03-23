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
                "The 'dirs-rs' crate failed to find the path for your platform ({})! {}",
                PLATFORM,
                "Maybe your platform isn't supported, report at 'https://codeberg.org/dirs/dirs-rs'."
            ),
        }
    }
}