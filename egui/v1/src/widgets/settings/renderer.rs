use std::fmt::Display;

use egui::{CursorIcon, Response, RichText, TextStyle, Ui, Vec2, emath::Numeric};

use crate::{ui_utils::combo_box::ui_strong_selectable_value, widgets::settings::section::Section};

pub struct SettingsRenderer {}

impl SettingsRenderer {
    pub fn show_combo_box<T: PartialEq + Display>(
        ui: &mut Ui,
        desired_widget_size: Vec2,
        choices: Vec<T>,
        config_key_path: &String,
        config_key_value: &mut T
    ) {
        ui.scope(|ui| {
            ui.style_mut().spacing.interact_size.y = desired_widget_size.y;

            // NOTE: hopefully developers don't decide to apply more than one section 
            // with the same config key and also decide to render a combo box on both of them 
            // in the future, as if they do... they will be met with a lovely surprise (YOU CAN'T!).
            let combo_box_id = ui.id()
                .with("selection_choices")
                .with(config_key_path);

            egui::ComboBox::from_id_salt(combo_box_id) 
                .selected_text(RichText::new(format!("{:#}", config_key_value)).heading())
                .width(desired_widget_size.x)
                .show_ui(ui, |ui| {
                    for choice in choices {
                        let choice_display = format!("{:#}", choice);

                        ui_strong_selectable_value::<T>(
                            ui,
                            config_key_value,
                            choice,
                            RichText::new(choice_display).heading()
                        );
                    }
                }
            ).response.on_hover_cursor(CursorIcon::PointingHand);
        });
    }

    pub fn show_int_drag_value<N: Numeric + Display>(ui: &mut Ui, desired_widget_size: Vec2, section: &mut Section<'_, N>) {
        let range = match &section.overrides.int_range {
            Some(int_range) => int_range.clone(),
            None => N::MIN..=N::MAX
        };

        match &section.overrides.choices {
            Some(choices) => SettingsRenderer::show_combo_box(
                ui,
                desired_widget_size,
                choices.clone(),
                &section.config_key_path,
                section.config_key_value,
            ),
            None => {
                ui.scope(|ui| {
                    // I think this is the only way to make the drag value text bigger.
                    ui.style_mut().drag_value_text_style = TextStyle::Heading;
        
                    // TODO: turn drag value into a custom cirrus egui widget.
                    let response = ui.add_sized(
                        desired_widget_size,
                        egui::DragValue::new(section.config_key_value)
                            .range(range)
                            .speed(0.2)
                    );
        
                    let cursor_icon = match response.dragged() {
                        true => CursorIcon::ResizeHorizontal,
                        false => CursorIcon::Text
                    };
        
                    response.on_hover_cursor(cursor_icon);
                });
            }
        };
    }
} 

