use cirrus_config::v1::{config::CConfig, template::Template};
use cirrus_theming::v1::theme::Theme;
use egui::{Color32, Context, CornerRadius, Frame, Key, Margin, RichText, Stroke, Ui};
use egui_notify::ToastLevel;

use crate::v1::{config_manager::{ConfigManager}, notifier::Notifier, widgets::settings::section::AnySection};

pub mod section;

pub struct Settings<'a> {
    sections: Vec<AnySection<'a>>
}

/// This widget is idempotent, meaning it can be initialized 
/// on every Egui frame and still handle state persistently and avoid a full re-init.
impl<'a> Settings<'a> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }

    pub fn add_section<T: Into<AnySection<'a>>>(&mut self, section: T) -> &mut Self {
        // if self.sections.iter().any(|random_section| *random_section == section) {
        //     return self;
        // }

        self.sections.push(section.into());

        self
    }

    /// Handles settings panel open and closing input as well as saving config on exit all for us.
    pub fn handle_input<T: CConfig>(
        ctx: &Context,
        config_manager: &mut ConfigManager<T>,
        notifier: &mut Notifier,
        show_state: &mut bool
    ) {
        let mut save_and_toast = |force_save: bool| {
            if force_save {
                match config_manager.save() {
                    Ok(_) => {
                        notifier.toast("Force saved config!", ToastLevel::Success, |_| {});
                    },
                    Err(error) => {
                        notifier.toast(error, ToastLevel::Error, |_| {});
                    }
                }

                return;
            }

            match config_manager.save_if_changed() {
                Ok(changed) => {
                    if changed {
                        notifier.toast("Config changes saved!", ToastLevel::Success, |_| {});
                    }
                },
                Err(error) => {
                    notifier.toast(error, ToastLevel::Error, |_| {});
                }
            }
        };

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) && *show_state {
            save_and_toast(true);
        }

        if ctx.input(|input| input.key_pressed(Key::Escape)) && *show_state {
            *show_state = false;

            save_and_toast(false);
        }

        // TODO: make this key bind customizable via the 
        // config in the future when we have a good key binds system.
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Comma)) {
            *show_state = !*show_state;

            if *show_state == false {
                save_and_toast(false);
            }
        }
    }

    pub fn show_ui(&mut self, ui: &mut Ui, theme: &Theme, template_config: &Template) {        
        ui.vertical_centered(|ui| {
            ui.set_max_width(ui.available_width().min(900.0));

            let primary_colour = Color32::from_hex(&theme.pallet.primary.to_hex_string()).unwrap();
            let surface_colour = Color32::from_hex(&theme.pallet.surface.to_hex_string()).unwrap();
            let text_colour = Color32::from_hex(&theme.pallet.text.to_hex_string()).unwrap();

            let settings_grid_colour = primary_colour.blend(
                surface_colour.gamma_multiply(0.2)
            );

            let settings_section_colour = settings_grid_colour.blend(Color32::GRAY.gamma_multiply(0.1));

            let settings_section_stroke = Stroke::new(1.0, text_colour.gamma_multiply(0.3));

            let grid = Frame::group(&ui.style())
                .corner_radius(CornerRadius {nw: 15, ne: 15, sw: 10, se: 10})
                .outer_margin(Margin::same(7))
                .stroke(Stroke::NONE)
                .fill(settings_grid_colour);

            egui::ScrollArea::vertical().show(ui, |ui| {
                grid.show(ui, |ui| {
                    ui.heading(RichText::new("Settings").strong().size(70.0));
                    ui.add_space(5.0);
                    ui.end_row();

                    if let Some(template_keys) = &template_config.keys {
                        for section in &mut self.sections {
                            let (section_display_info, section_config_key_path) = match section {
                                AnySection::String(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::Bool(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntTiny(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntSmall(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntBig(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::FloatSmall(section) => (section.display_info.clone(), section.config_key_path.clone()),
                            };

                            let (toml_config_key, config_docstring) = match template_keys.get(&section_config_key_path) {
                                None => (
                                    &section_config_key_path.split(".").last().expect(
                                        "Failed to split section config key path! This is odd as the \
                                        'config_key_path!()' macro should always return a dotted key path."
                                    ).to_string(),
                                    &section_display_info.description.unwrap_or_default()
                                ),
                                Some(template_key) => {
                                    (
                                        &template_key.key,
                                        // NOTE: if there's no description we default to empty string for now, 
                                        // in the future we will reflect that appropriately in the settings UI.
                                        &template_key.docstring.description.long.clone().unwrap_or_default()
                                    )
                                }
                            };

                            let config_title = match &section_display_info.name {
                                Some(name) => name.clone(),
                                None => {
                                    let title: &String = &toml_config_key
                                        .replace("_", " ")
                                        .split_whitespace()
                                        .map(|word| {
                                            let mut chars = word.chars();
                                            match chars.next() {
                                                Some(first_char) => format!(
                                                    "{}{} ",
                                                    first_char.to_uppercase().collect::<String>(),
                                                    chars.as_str()
                                                ),
                                                None => String::new(),
                                            }
                                        })
                                        .collect();
                    
                                    let mut title = title.clone();
                                    title.pop();
                    
                                    title
                                },
                            };

                            Frame::group(ui.style())
                                .stroke(settings_section_stroke)
                                .outer_margin(Margin { top: 7, ..Default::default() })
                                .inner_margin(Margin { left: 12, right: 12, top: 4, bottom: 8 })
                                .fill(settings_section_colour)
                                .show(ui, |ui|{
                                    Self::render_section(ui, section, &config_title, config_docstring);
                                });

                            ui.end_row();
                        }
                    }
                });
            });
        });
    }
}