use std::io;
use std::{env, path::Path};
use std::process::{Command, ExitStatus};

use crate::v1::error::{Error, Result};

pub mod error;

pub enum Preference {
    None,
    Terminal,
}

impl Default for Preference {
    fn default() -> Self { Self::None }
}

// NOTE: maybe expand this in the future

pub fn open_editor(file_path: &Path, pref: Preference) -> Result<ExitStatus> {
    let editor_env_result = env::var("VISUAL")
        .or_else(|_| env::var("EDITOR"));

    let status_error_map = |error: io::Error| {
        Error::EditorCouldNotOpenFailure { error: error.to_string() }
    };

    if let Ok(editor) = editor_env_result {
        return Ok(
            Command::new(editor)
                .arg(file_path)
                .status()
                .map_err(status_error_map)?
        );
    }

    // TODO: windows and macos needs testing..

    #[cfg(target_os = "windows")]
    {
        Ok(
            Command::new("notepad")
                .arg(file_path)
                .status()
                .map_err(status_error_map)?
        )
    }

    #[cfg(target_os = "macos")]
    {
        Ok(
            Command::new("open")
                .arg(file_path)
                .status()
                .map_err(status_error_map)?
        )
    }

    #[cfg(target_os = "linux")]
    {
        match pref {
            Preference::None => Ok(
                Command::new("xdg-open")
                    .arg(file_path)
                    .status()
                    .map_err(status_error_map)?
            ),
            Preference::Terminal => Ok(
                Command::new("nano")
                    .arg(file_path)
                    .status()
                    .map_err(status_error_map)?
            ),
        }
    }
}