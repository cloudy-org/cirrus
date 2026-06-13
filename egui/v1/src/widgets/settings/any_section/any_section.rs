use cirrus_config::template::TemplateKeys;
use egui::{Color32, Frame, RichText, Separator, Stroke, TextWrapMode, Ui, Vec2};

use crate::widgets::{buttons::toggle_button::ToggleButton, settings::{any_section::renderer::SettingsRenderer, section::{Section, SectionDisplayInfo}}};

pub enum AnySection<'a> {
    String(Section<'a, String>),
    OptionalString(Section<'a, Option<String>>),
    Bool(Section<'a, bool>),
    IntTiny(Section<'a, u8>),
    IntSmall(Section<'a, i32>),
    FloatSmall(Section<'a, f32>),
    IntBig(Section<'a, i64>),

    ChildSections {
        title: String,
        sections: Vec<AnySection<'a>>
    }
}

impl<'a> From<Section<'a, String>> for AnySection<'a> {
    fn from(section: Section<'a, String>) -> Self {
        AnySection::String(section)
    }
}

impl<'a> From<Section<'a, Option<String>>> for AnySection<'a> {
    fn from(section: Section<'a, Option<String>>) -> Self {
        AnySection::OptionalString(section)
    }
}

impl<'a> From<Section<'a, bool>> for AnySection<'a> {
    fn from(section: Section<'a, bool>) -> Self {
        AnySection::Bool(section)
    }
}

impl<'a> From<Section<'a, u8>> for AnySection<'a> {
    fn from(section: Section<'a, u8>) -> Self {
        AnySection::IntTiny(section)
    }
}

impl<'a> From<Section<'a, i32>> for AnySection<'a> {
    fn from(section: Section<'a, i32>) -> Self {
        AnySection::IntSmall(section)
    }
}

impl<'a> From<Section<'a, f32>> for AnySection<'a> {
    fn from(section: Section<'a, f32>) -> Self {
        AnySection::FloatSmall(section)
    }
}

impl<'a> From<Section<'a, i64>> for AnySection<'a> {
    fn from(section: Section<'a, i64>) -> Self {
        AnySection::IntBig(section)
    }
}

// impl<'a> From<Vec<AnySection<'a>>> for AnySection<'a> {
//     fn from(sections: Vec<AnySection<'a>>) -> Self {
//         AnySection::ChildSections(sections)
//     }
// }

// impl<'a, T> From<Vec<Section<'a, T>>> for AnySection<'a>
//     where AnySection<'a>: From<Section<'a, T>>
// {
//     fn from(sections: Vec<Section<'a, T>>) -> Self {
//         let any_sections: Vec<AnySection<'a>> = sections.into_iter()
//             .map(|section| section.into())
//             .collect();

//         AnySection::ChildSections(any_sections)
//     }
// }

impl AnySection<'_> {
    pub(crate) fn show(
        &mut self,
        ui: &mut Ui,
        child_surface_colour: &Color32,
        template_keys: &TemplateKeys,
        is_child: bool,
    ) {
        let (heading_size, widget_size) = match is_child {
            true => (18.0, Vec2::new(4.0, 2.0)),
            false => (24.0, Vec2::new(5.0, 2.8))
        };

        ui.heading(RichText::new(self.get_title()).size(heading_size));

        ui.horizontal(|ui| {
            let desired_widget_size = ui.spacing().interact_size.y * widget_size;

            match self {
                AnySection::String(section) => {
                    match &section.overrides.choices {
                        Some(choices) => SettingsRenderer::show_combo_box(
                            ui,
                            desired_widget_size,
                            choices.clone(),
                            &section.config_key_path,
                            section.config_key_value
                        ),
                        None => SettingsRenderer::show_text_input(
                            ui,
                            desired_widget_size,
                            section.overrides.text_edit_placeholder.clone(),
                            section.config_key_value
                        )
                    };
                },
                AnySection::OptionalString(section) => {
                    match section.config_key_value {
                        Some(config_value) => {
                            match &section.overrides.choices {
                                Some(choices) => SettingsRenderer::show_combo_box(
                                    ui,
                                    desired_widget_size,
                                    choices.iter()
                                        .filter_map(|choice| choice.clone())
                                        .collect(),
                                    &section.config_key_path,
                                    config_value
                                ),
                                None => SettingsRenderer::show_text_input(
                                    ui,
                                    desired_widget_size,
                                    section.overrides.text_edit_placeholder.clone(),
                                    config_value
                                )
                            };
                        },
                        // TODO: we need to add a button to set this optional 
                        // config key to None or back to Some (a value). I'll leave 
                        // this task for someone else as I cannot figure it out right away.
                        None => {
                            ui.add_enabled_ui(false, |ui| {
                                SettingsRenderer::show_combo_box(
                                    ui,
                                    desired_widget_size,
                                    Vec::new(),
                                    &section.config_key_path,
                                    &mut "NONE"
                                );
                            }).response.on_disabled_hover_text(
                                format!(
                                    "Optional config key '{}' is not present in the config file!",
                                    section.config_key_path
                                )
                            );
                        },
                    }
                },
                AnySection::Bool(section) => {
                    ToggleButton::new(&mut section.config_key_value)
                        .size(widget_size)
                        .show(ui);
                },
                AnySection::IntTiny(section) => {
                    let slider_range = match &section.overrides.int_range {
                        Some(int_range) => int_range.clone(),
                        None => u8::MIN..=u8::MAX
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
                                // Set slider width to desired widget width
                                ui.style_mut().spacing.slider_width = desired_widget_size.x;
        
                                ui.add(
                                    egui::Slider::new(
                                        section.config_key_value,
                                        slider_range
                                    )
                                );
                            });
                        }
                    };
                },
                AnySection::IntSmall(section) => SettingsRenderer::show_int_drag_value(ui, desired_widget_size, section),
                AnySection::FloatSmall(section) => SettingsRenderer::show_int_drag_value(ui, desired_widget_size, section),
                AnySection::IntBig(section) => SettingsRenderer::show_int_drag_value(ui, desired_widget_size, section),
                AnySection::ChildSections { sections, .. } => {
                    let child_grid = Frame::group(ui.style())
                        .stroke(Stroke::NONE)
                        .fill(*&child_surface_colour.gamma_multiply(0.4));

                    child_grid.show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.scope(|ui| {
                                ui.spacing_mut().item_spacing.y = 8.0;

                                let last_section_index = sections.len() - 1;

                                for (index, any_section) in sections.into_iter().enumerate() {
                                    Frame::group(ui.style())
                                        .stroke(Stroke::NONE)
                                        .show(ui, |ui|{
                                            any_section.show(
                                                ui,
                                                child_surface_colour,
                                                template_keys,
                                                true,
                                            );

                                            ui.take_available_width();
                                        });

                                    ui.end_row();

                                    if index == last_section_index {
                                        continue;
                                    }

                                    ui.add(Separator::default().shrink(5.0));
                                    ui.end_row();
                                }
                            });
                        });

                        ui.take_available_width();
                    });

                    ui.end_row();

                    return;
                }
            }

            ui.add_space(1.0);

            ui.separator();
            ui.add_space(1.0);

            ui.add(
                egui::Label::new(
                    match self.get_description(template_keys) {
                        Some(description) => description,
                        // TODO: something better than just no description text
                        None => String::from("No Description"),
                    }
                ).wrap_mode(TextWrapMode::Wrap)
            );
        });
    }

    fn get_title(&self) -> String {
        let config_key_path = match self {
            AnySection::String(section) => &section.config_key_path,
            AnySection::OptionalString(section) => &section.config_key_path,
            AnySection::Bool(section) => &section.config_key_path,
            AnySection::IntTiny(section) => &section.config_key_path,
            AnySection::IntSmall(section) => &section.config_key_path,
            AnySection::FloatSmall(section) => &section.config_key_path,
            AnySection::IntBig(section) => &section.config_key_path,
            AnySection::ChildSections { title, .. } => return title.clone(),
        };

        let final_config_key = config_key_path
            .split(".")
            .last()
            .expect(
                "Unexpected config key path string! Expected period separated \
                key path. You should be using the 'config_key_path!()' macro."
            )
            .to_string();

        let display_info = self.get_display_info();

        match display_info.name {
            Some(name) => name,
            None => {
                let mut title: String = final_config_key
                    .replace("_", " ")
                    .split_whitespace()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            Some(first_char) => format!(
                                "{}{} ",
                                first_char.to_uppercase().to_string(),
                                chars.as_str()
                            ),
                            None => String::new(),
                        }
                    })
                    .collect();

                title.pop();

                title
            },
        }
    }

    fn get_description<'a>(&'a self, template_keys: &TemplateKeys) -> Option<String> {
        let config_key_path = match self {
            AnySection::String(section) => &section.config_key_path,
            AnySection::OptionalString(section) => &section.config_key_path,
            AnySection::Bool(section) => &section.config_key_path,
            AnySection::IntTiny(section) => &section.config_key_path,
            AnySection::IntSmall(section) => &section.config_key_path,
            AnySection::FloatSmall(section) => &section.config_key_path,
            AnySection::IntBig(section) => &section.config_key_path,
            AnySection::ChildSections { .. } => return None,
        };

        let display_info = self.get_display_info();

        match template_keys.get(config_key_path) {
            Some(template_key) => template_key.docstring.description.long.clone(),
            None => display_info.description
        }
    }

    // TODO: return SectionDisplayInfo as a reference
    fn get_display_info(&self) -> SectionDisplayInfo {
        match self {
            AnySection::String(section) => section.display_info.clone(),
            AnySection::OptionalString(section) => section.display_info.clone(),
            AnySection::Bool(section) => section.display_info.clone(),
            AnySection::IntTiny(section) => section.display_info.clone(),
            AnySection::IntSmall(section) => section.display_info.clone(),
            AnySection::FloatSmall(section) => section.display_info.clone(),
            AnySection::IntBig(section) => section.display_info.clone(),
            AnySection::ChildSections { .. } => SectionDisplayInfo::default(),
        }
    }
}