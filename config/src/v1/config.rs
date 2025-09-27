use std::{fs, hash::Hash};

use log::debug;
use cirrus_path::v1::{get_user_config_dir_path};
use cirrus_error::v1::error::CError;
use serde::{de::DeserializeOwned, Serialize};

use crate::v1::error::Error;

pub trait CConfig: DeserializeOwned + Serialize + Hash + Default {}

pub fn get_and_create_config_file<T: CConfig>(app_name: &str, template_config_toml_string: &str) -> Result<T, Box<dyn CError>> {
    let roseate_config_dir_path = get_user_config_dir_path(&app_name)?;

    if !roseate_config_dir_path.exists() {
        debug!("Config directory missing ({}), creating dir for '{}'...", roseate_config_dir_path.display(), app_name);

        if let Err(error) = fs::create_dir_all(&roseate_config_dir_path) {
            return Err(Box::new(Error::FailedToCreateConfigDirectory(error.to_string())));
        };

        debug!("Config directory created!");
    }

    let toml_config_path = roseate_config_dir_path.join("config.toml");

    if toml_config_path.exists() {
        debug!("Reading and deserializing config file...");

        return match fs::read_to_string(&toml_config_path) {
            Ok(value) => Ok(
                toml::from_str::<T>(&value)
                    .map_err(|error| Error::FailedToReadConfig(error.to_string()))?
            ),
            Err(error) => Err(Error::FailedToReadConfig(error.to_string()).into()),
        };
    }

    debug!(
        "Reading template config and creating config file at '{}'...", 
        &toml_config_path.display()
    );

    let result = fs::write(
        &toml_config_path, template_config_toml_string
    );

    match result {
        Ok(_) => Ok(
            toml::from_str(template_config_toml_string)
                .expect("Failed to deserialize template toml file!")
            // I'm panicking here as if this fails to deserialize it's our fault!
            // Tests should be put in place to ensure this doesn't happen from our end.
            // 
            // TODO: Make a cargo test to confirm the config.template.toml 
            // deserializes without error. Then also add it as a github workflow.
        ),
        Err(error) => Err(Error::FailedToWriteToConfig(error.to_string()).into())
    }
}