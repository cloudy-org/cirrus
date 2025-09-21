use std::{fs, hash::{DefaultHasher, Hasher}, time::Duration};

use cirrus_config::v1::{config::{get_and_create_config_file, CConfig}, error::Error as ConfigError};
use cirrus_error::v1::error::CError;
use cirrus_path::v1::get_user_config_dir_path;
use egui::Context;
use egui_notify::ToastLevel;
use log::debug;
use toml_edit::{Document, DocumentMut, Item, Table};

use crate::v1::{notifier::Notifier, scheduler::Scheduler};

pub struct ConfigManager<T: CConfig> {
    pub config: T,
    last_config_hash: u64,

    config_disk_string_copy: Option<String>,

    config_autosave_schedule: Option<Scheduler>
}

impl<'a, T: CConfig> Default for ConfigManager<T> {
    fn default() -> Self {
        Self {
            config: Default::default(),
            last_config_hash: 0, // hopefully this doesn't break any logic I'm about to write
            config_disk_string_copy: None,
            config_autosave_schedule: None
        }
    }
}

impl<'a, T: CConfig> ConfigManager<T> {
    pub fn new(app_name: &str, template_config_toml_string: &'a str) -> Result<Self, Box<dyn CError>> {
        let config: T = get_and_create_config_file(app_name, template_config_toml_string)?;

        let mut hasher = DefaultHasher::new();
        config.hash(&mut hasher);

        let config_hash = hasher.finish();

        let config_path = get_user_config_dir_path(app_name)?.join("config.toml");
        // Reading the config here really shouldn't fail as that would be caught by 
        // "get_and_create_config_file" but I guess just to be extra safe and panic-less 
        // let's map it to the same exact error.
        let copy_of_config_on_disk = fs::read_to_string(config_path)
            .map_err(|error| ConfigError::FailedToReadConfig(error.to_string()))?;

        Ok(
            Self {
                config,
                last_config_hash: config_hash,
                config_disk_string_copy: Some(copy_of_config_on_disk),
                config_autosave_schedule: None
            }
        )
    }

    pub fn update<E: CError>(&mut self, ctx: &Context, notifier: &mut Notifier<E>) {
        if self.config_autosave_schedule.is_none() {
            self.config_autosave_schedule = Some(Scheduler::new(|| {}, Duration::from_secs(4)));
        }

        if let Some(ref mut config_autosave_schedule) = &mut self.config_autosave_schedule {
            if config_autosave_schedule.update().is_some() {
                // We're using hashes to detect changes to the config struct.
                let mut hasher = DefaultHasher::new();
                self.config.hash(&mut hasher);

                let current_config_hash = hasher.finish();

                if current_config_hash != self.last_config_hash {
                    self.save(); // TODO: handle errors in result

                    notifier.toast(
                        "Config has been autosaved!",
                        ToastLevel::Success,
                        |_| {}
                    );

                    self.last_config_hash = current_config_hash;
                }

                self.config_autosave_schedule = Some(Scheduler::new(|| {}, Duration::from_secs(4)));
            }
        };
    }

    /// Writes the mutated config we currently have in memory to the user's config file in disk.
    pub fn save(&self) -> Result<(), Box<dyn CError>> {
        debug!("Saving mutated config to disk...");

        // TODO: now we need to make sure when a user updates the disk variant of 
        // the config we have a watchdog that tells us so we can update our disk copy.
        if let Some(config_disk_copy) = &self.config_disk_string_copy {
            debug!("Serializing config in memory into a string to prepare for walking and editing disk config...");

            // I'm expecting here as the in memory config should really be safe from 
            // serialization failures or at least I can't imagine it failing... imagine it does...
            let updated_config_string = toml::to_string(&self.config)
                .expect("Failed to serialize config in memory for saving to disk!");

            debug!("Serializing config in memory into into toml document...");
            let updated_config_document: Document<String> = updated_config_string.parse().unwrap();

            debug!("Serializing copy of config in disk into into toml document...");
            // Unwrapping for now because we already parsed and handled errors for this hence it shouldn't fail.
            let mut config_to_write_to_disk_document = config_disk_copy.parse::<DocumentMut>().unwrap();

            debug!("Walking and editing disk toml document...");
            self.walk_and_edit_toml_document(
                &updated_config_document,
                None,
                &mut config_to_write_to_disk_document
            );

            println!("DISK CONFIG -> {}", config_to_write_to_disk_document);

            // TODO: write to config in disk
        }

        Ok(())
    }

    fn walk_and_edit_toml_document(
        &self,
        table_to_walk: &Table,
        key_path: Option<&String>,
        document_to_edit: &mut DocumentMut
    ) {
        for (key, item) in table_to_walk.iter() {
            let key_path = match key_path {
                Some(path) => &format!("{}.{}", path, key),
                None => &key.to_string(),
            };

            match item {
                Item::None => return,
                Item::Value(value) => {
                    let previous_item = Self::get_toml_item_by_path(&document_to_edit, key_path).unwrap();

                    let string_value = item.to_string();
                    let previous_string_value = previous_item.to_string();

                    if string_value != previous_string_value {
                        document_to_edit[key] = Item::Value(value.clone());
                    }
                },
                Item::Table(table) => {
                    self.walk_and_edit_toml_document(table, Some(key_path), document_to_edit);
                },
                Item::ArrayOfTables(array_of_tables) => {
                    // TODO: this needs testing!

                    for child_table in array_of_tables {
                        self.walk_and_edit_toml_document(
                            child_table, Some(key_path), document_to_edit
                        );
                    }

                },
            }
        }
    }

    fn get_toml_item_by_path(document: &'a DocumentMut, key_path: &String) -> Option<&'a Item> {
        let mut document: &Item = document.as_item();

        for part in key_path.split('.') {
            if let Some(table_like) = document.as_table_like() {
                document = table_like.get(part)?;
            } else {
                return None;
            }
        }

        Some(document)
    }

    // fn is_toml_value_different(value_a: &Value, value_b: &Value) -> bool {
    //     match (value_a, value_b) {
    //         (Value::String(formatted_a), Value::String(formatted_b)) => !(formatted_a == formatted_b),
    //         (Value::Integer(formatted_a), Value::Integer(formatted_b)) => !(formatted_a == formatted_b),
    //         (Value::Float(formatted_a), Value::Float(formatted_b)) => !(formatted_a == formatted_b),
    //         (Value::Boolean(formatted_a), Value::Boolean(formatted_b)) => !(formatted_a == formatted_b),
    //         (Value::Datetime(formatted_a), Value::Datetime(formatted_b)) => !(formatted_a == formatted_b),
    //         (Value::Array(array), Value::Array(previous_array)) => {
    //             let mut is_different = array.len() != previous_array.len();

    //             if is_different == false {
    //                 let mut iter_index = 0;

    //                 is_different = array.iter()
    //                     .all(|value| {
    //                         let previous_value = previous_array.get(iter_index);

    //                         iter_index += 1;

    //                         match previous_value {
    //                             Some(previous_value) => Self::is_toml_value_different(value, previous_value),
    //                             None => true,
    //                         }
    //                     }
    //                 );
    //             }

    //             is_different
    //         },
    //         (Value::InlineTable(inline_table), Value::InlineTable(previous_inline_table)) => {
    //             let mut is_different = inline_table.len() != previous_inline_table.len();

    //             if is_different == false {
    //                 is_different = inline_table.iter()
    //                     .all(|(key, value)| {
    //                         let previous_value = previous_inline_table.get(key);

    //                         match previous_value {
    //                             Some(previous_value) => Self::is_toml_value_different(value, previous_value),
    //                             None => true,
    //                         }
    //                     }
    //                 );
    //             }

    //             is_different
    //         },
    //         _ => false,
    //     }
    // }
}