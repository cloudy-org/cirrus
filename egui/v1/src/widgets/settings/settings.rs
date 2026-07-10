use cirrus_theming::theme::Theme;
use cirrus_config::{config::CConfig, template::TemplateKeys};

use egui_notify::ToastLevel;
use egui::{Color32, Context, CornerRadius, Frame, Key, Margin, Modifiers, RichText, Stroke, Ui};

use crate::{config_manager::ConfigManager, notifier::{Notifier, toast::ToastText}, widgets::settings::any_section::AnySection};

pub struct Settings<'a> {
    any_sections: Vec<AnySection<'a>>
}

/// This widget is idempotent, meaning it can be initialized on every 
/// Egui frame and still handle state persistently and avoid a full re-init.
impl<'a> Settings<'a> {
    pub fn new() -> Self {
        Self {
            any_sections: Vec::new(),
        }
    }

    pub fn add_section<T: Into<AnySection<'a>>>(&mut self, section: T) -> &mut Self {
        self.any_sections.push(section.into());

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
                        notifier.show_toast("Force saved config!", ToastLevel::Success, |_| {});
                    },
                    Err(error) => {
                        notifier.show_toast(ToastText::Error(error.into()), ToastLevel::Error, |_| {});
                    }
                }

                return;
            }

            match config_manager.save_if_changed() {
                Ok(changed) => {
                    if changed {
                        notifier.show_toast("Config changes saved!", ToastLevel::Success, |_| {});
                    }
                },
                Err(error) => {
                    notifier.show_toast(ToastText::Error(error.into()), ToastLevel::Error, |_| {});
                }
            }
        };

        if *show_state && ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) {
            save_and_toast(true);
        }

        if *show_state && ctx.input_mut(|input| input.consume_key(Modifiers::default(), Key::Escape)) {
            *show_state = false;

            save_and_toast(false);
        }

        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Comma)) {
            *show_state = !*show_state;

            if *show_state == false {
                save_and_toast(false);
            }
        }
    }

    pub fn show_ui(&mut self, ui: &mut Ui, theme: &Theme, template_keys: &TemplateKeys) {        
        ui.vertical_centered(|ui| {
            ui.set_max_width(ui.available_width().min(900.0));

            let surface_colour = Color32::from_hex(&theme.palette.surface.to_hex_string()).unwrap();
            let text_colour = Color32::from_hex(&theme.palette.text.to_hex_string()).unwrap();

            // blend surface colour with the background colour (which is the primary colour).
            let settings_grid_colour = surface_colour.gamma_multiply(0.2);

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

                    for any_section in &mut self.any_sections {
                        Frame::group(ui.style())
                            .stroke(settings_section_stroke)
                            .outer_margin(Margin { top: 7, ..Default::default() })
                            .inner_margin(Margin { left: 12, right: 12, top: 4, bottom: 8 })
                            .fill(settings_section_colour)
                            .show(ui, |ui| {
                                any_section.show(
                                    ui,
                                    &surface_colour,
                                    template_keys,
                                    false
                                );
                            });

                        ui.end_row();
                    }
                });
            });
        });
    }
}