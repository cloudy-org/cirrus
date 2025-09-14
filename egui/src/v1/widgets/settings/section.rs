use std::{fmt::Display, ops::RangeInclusive};

use egui::{emath::Numeric, CursorIcon, RichText, TextEdit, TextStyle, TextWrapMode, Ui, Vec2};

use crate::v1::{ui_utils::combo_box::ui_strong_selectable_value, widgets::{buttons::toggle_button::ToggleButton, settings::Settings}};

pub struct Section<'a, T> {
    pub(crate) config_key: &'a mut T,
    pub(crate) config_key_path: String,

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
    pub description: Option<String>
}

impl<'a, T> Section<'a, T> {
    pub fn new(
        config_key_path: String,
        config_key: &'a mut T,
        overrides: SectionOverrides<'a, T>,
        display_info: SectionDisplayInfo
    ) -> Self {
        Self {
            config_key,
            config_key_path,
            overrides,
            display_info,
        }
    }
}

// NOTE: umm do we need more types???
// We'll add more when necessary as we run into them.
pub enum AnySection<'a> {
    String(Section<'a, String>),
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

impl Settings<'_> {
    pub(super) fn render_section(ui: &mut Ui, section: &mut AnySection<'_>, config_title: &String, config_docstring: &String) {
        ui.heading(RichText::new(config_title).strong());

        ui.horizontal(|ui| {
            let widget_size = Vec2::new(6.0, 3.0);
            let desired_widget_size = ui.spacing().interact_size.y * widget_size;

            match section {
                AnySection::String(section) => {
                    let text_edit_placeholder = match &section.overrides.text_edit_placeholder {
                        Some(placeholder) => placeholder,
                        None => "Enter string here..."
                    };

                    match &section.overrides.choices {
                        Some(choices) => Self::render_combo_box(ui, desired_widget_size, choices.clone(), section),
                        None => {
                            ui.add(
                                TextEdit::multiline(section.config_key)
                                    .desired_width(desired_widget_size.x + 100.0)
                                    .hint_text(text_edit_placeholder)
                                    .font(TextStyle::Heading)
                                    .return_key(None)
                            );
                        }
                    };
                },
                AnySection::Bool(section) => {
                    ToggleButton::new(&mut section.config_key)
                        .size(widget_size)
                        .show(ui);
                },
                AnySection::IntTiny(section) => {
                    let slider_range = match &section.overrides.int_range {
                        Some(int_range) => int_range.clone(),
                        None => u8::MIN..=u8::MAX
                    };

                    match &section.overrides.choices {
                        Some(choices) => Self::render_combo_box(ui, desired_widget_size, choices.clone(), section),
                        None => {
                            ui.scope(|ui| {
                                // Set slider width to desired widget width
                                ui.style_mut().spacing.slider_width = desired_widget_size.x;
        
                                ui.add(
                                    egui::Slider::new(section.config_key, slider_range)
                                );
                            });
                        }
                    };
                },
                AnySection::IntSmall(section) => Self::render_int_drag_value(ui, desired_widget_size, section),
                AnySection::FloatSmall(section) => Self::render_int_drag_value(ui, desired_widget_size, section),
                AnySection::IntBig(section) => Self::render_int_drag_value(ui, desired_widget_size, section)
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

    fn render_combo_box<T: PartialEq + Display>(ui: &mut Ui, desired_widget_size: Vec2, choices: Vec<T>, section: &mut Section<'_, T>) {
        ui.scope(|ui| {
            ui.style_mut().spacing.interact_size.y = desired_widget_size.y;

            // NOTE: hopefully developers don't decide to apply more than one section 
            // with the same config key and also decide to render a combo box on both of them 
            // in the future, as if they do... they will be met with a lovely surprise (YOU CAN'T!).
            let combo_box_id = ui.id()
                .with("selection_choices")
                .with(&section.config_key_path);

            egui::ComboBox::from_id_salt(combo_box_id) 
                .selected_text(RichText::new(format!("{:#}", section.config_key)).heading())
                .width(desired_widget_size.x)
                .show_ui(ui, |ui| {
                    for choice in choices {
                        let choice_display = format!("{:#}", choice);

                        ui_strong_selectable_value::<T>(
                            ui,
                            section.config_key,
                            choice,
                            RichText::new(choice_display).heading()
                        );
                    }
                }
            );
        });
    }

    fn render_int_drag_value<N: Numeric + Display>(ui: &mut Ui, desired_widget_size: Vec2, section: &mut Section<'_, N>) {
        let range = match &section.overrides.int_range {
            Some(int_range) => int_range.clone(),
            None => N::MIN..=N::MAX
        };

        match &section.overrides.choices {
            Some(choices) => Self::render_combo_box(ui, desired_widget_size, choices.clone(), section),
            None => {
                ui.scope(|ui| {
                    // I think this is the only way to make the drag value text bigger.
                    ui.style_mut().drag_value_text_style = TextStyle::Heading;
        
                    // TODO: turn drag value into a custom cirrus egui widget.
                    let response = ui.add_sized(
                        desired_widget_size,
                        egui::DragValue::new(section.config_key)
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