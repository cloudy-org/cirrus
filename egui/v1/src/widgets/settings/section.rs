use std::{ops::RangeInclusive};

use egui::{RichText, TextEdit, TextStyle, TextWrapMode, Ui, Vec2};

use crate::{widgets::{buttons::toggle_button::ToggleButton, settings::{renderer::SettingsRenderer}}};
pub struct Section<'a, T> {
    pub(crate) config_key_path: String,
    pub(crate) config_key_value: &'a mut T,

    pub overrides: SectionOverrides<'a, T>,

    pub display_info: SectionDisplayInfo
}

/// Struct that allows the developer to override and customize 
/// default values and **constraints** that would be set by section widgets.
#[derive(Default)]
pub struct SectionOverrides<'a, T> {
    pub choices: Option<Vec<T>>,
    pub int_range: Option<RangeInclusive<T>>,
    pub text_edit_placeholder: Option<&'a str>
}

#[derive(Default, Clone)]
pub struct SectionDisplayInfo {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl<'a, T> Section<'a, T> {
    pub fn new(
        config_key_path: String,
        config_key_value: &'a mut T,
        overrides: SectionOverrides<'a, T>,
        display_info: SectionDisplayInfo
    ) -> Self {
        Self {
            config_key_path,
            config_key_value,
            overrides,
            display_info,
        }
    }
}

// NOTE: umm do we need more types??? We'll add more when necessary as we run into them.
// TODO: Add support for "ChildSections" that allows for multiple sections under a section.
pub enum AnySection<'a> {
    String(Section<'a, String>),
    OptionalString(Section<'a, Option<String>>),
    Bool(Section<'a, bool>),
    IntTiny(Section<'a, u8>),
    IntSmall(Section<'a, i32>),
    FloatSmall(Section<'a, f32>),
    IntBig(Section<'a, i64>),
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

impl AnySection<'_> {
    pub(super) fn show(
        &mut self,
        ui: &mut Ui,
        config_title: &String,
        config_docstring: &String,
    ) {
        ui.heading(RichText::new(config_title).strong());

        ui.horizontal(|ui| {
            let widget_size = Vec2::new(6.0, 3.0);
            let desired_widget_size = ui.spacing().interact_size.y * widget_size;

            match self {
                AnySection::String(section) => {
                    let text_edit_placeholder = match &section.overrides.text_edit_placeholder {
                        Some(placeholder) => placeholder,
                        None => "Enter string here..."
                    };

                    match &section.overrides.choices {
                        Some(choices) => SettingsRenderer::show_combo_box(
                            ui,
                            desired_widget_size,
                            choices.clone(),
                            &section.config_key_path,
                            section.config_key_value
                        ),
                        None => {
                            ui.add(
                                TextEdit::multiline(section.config_key_value)
                                    .desired_width(desired_widget_size.x + 100.0)
                                    .hint_text(text_edit_placeholder)
                                    .font(TextStyle::Heading)
                                    .return_key(None)
                            );
                        }
                    };
                },
                AnySection::OptionalString(section) => {
                    let text_edit_placeholder = match &section.overrides.text_edit_placeholder {
                        Some(placeholder) => placeholder,
                        None => "Enter string here..."
                    };

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
                                None => {
                                    ui.add(
                                        TextEdit::multiline(config_value)
                                            .desired_width(desired_widget_size.x + 100.0)
                                            .hint_text(text_edit_placeholder)
                                            .font(TextStyle::Heading)
                                            .return_key(None)
                                    );
                                }
                            };
                        },
                        // TODO: we need to add a button to set this optional 
                        // config key to none or back to a value. I'll leave this 
                        // task for someone else as I cannot figure it out right away.
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
                AnySection::IntBig(section) => SettingsRenderer::show_int_drag_value(ui, desired_widget_size, section)
            }

            ui.add_space(3.0);

            ui.separator();
            ui.add_space(3.0);

            ui.add(
                egui::Label::new(config_docstring)
                    .wrap_mode(TextWrapMode::Wrap)
            );
        });
    }
}