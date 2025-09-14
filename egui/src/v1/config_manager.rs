use std::time::Duration;

use cirrus_config::v1::config::{get_and_create_config_file, CConfig};
use cirrus_error::v1::error::CError;
use egui::Context;

use crate::v1::scheduler::Scheduler;

macro_rules! new_autosave_scheduler {
    () => {
        Scheduler::new(|| {}, Duration::from_secs(3))
    };
}

pub struct ConfigManager<T: CConfig> {
    pub config: T,

    config_autosave_schedule: Scheduler
}

impl<'a, T: CConfig> Default for ConfigManager<T> {
    fn default() -> Self {
        Self {
            config: Default::default(),
            config_autosave_schedule: new_autosave_scheduler!()
        }
    }
}

impl<'a, T: CConfig> ConfigManager<T> {
    pub fn new(app_name: &str, template_config_toml_string: &'a str) -> Result<Self, Box<dyn CError>> {
        let config = get_and_create_config_file(app_name, template_config_toml_string)?;

        Ok(
            Self {
                config,
                config_autosave_schedule: new_autosave_scheduler!()
            }
        )
    }

    pub fn update(&mut self, ctx: &Context) {
        if self.config_autosave_schedule.update().is_some() {
            self.config_autosave_schedule = new_autosave_scheduler!();
        }
    }
}