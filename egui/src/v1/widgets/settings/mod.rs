use std::collections::HashMap;

use cirrus_config::v1::config::CConfig;
use cirrus_theming::v1::theme::Theme;
use egui::{Color32, Context, CornerRadius, Frame, Id, Key, Margin, RichText, Stroke, Ui};
use egui_notify::ToastLevel;
use log::debug;
use toml_edit::{Document, Item, Table, Value};

use crate::v1::{config_manager::{ConfigManager}, notifier::Notifier, widgets::settings::section::AnySection};

pub mod section;

#[derive(Debug, Clone)]
struct TomlItem {
    key: String,
    value: Value
}

type Items<'a> = HashMap<String, TomlItem>;
type DocItems<'a> = HashMap<String, (TomlItem, String)>;

pub struct Settings<'a> {
    id: Id,

    config_items: Option<Items<'a>>,
    template_config_toml_string: &'a str,

    sections: Vec<AnySection<'a>>
}

/// This widget is idempotent, meaning it can be initialized 
/// on every Egui frame and still handle state persistently and avoid a full re-init.
impl<'a> Settings<'a> {
    pub fn new(template_config_toml_string: &'a str, ui: &Ui) -> Self {
        let id = ui.make_persistent_id("settings_widget");

        // let config_id = ui.make_persistent_id((id, "config_items"));
        // let memorized_config_items = ui.memory_mut(
        //     |mem| mem.data.get_persisted::<Option<Items>>(config_id)
        // ).unwrap_or_default();

        Self {
            id,

            config_items: None, // NOTE: not used yet
            template_config_toml_string,

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

    fn update(&self, ui: &mut Ui) {
        let template_config_id = ui.make_persistent_id((self.id, "template_config_items"));

        let mut memorized_template_config_items = ui.memory_mut(
            |mem| mem.data.get_persisted::<Option<DocItems>>(template_config_id)
        ).unwrap_or_default();

        if memorized_template_config_items.is_none() {
            debug!("Parsing template config items...");
            let mut template_config_items: DocItems = HashMap::new();

            let toml_string = self.template_config_toml_string;

            let document: Document<String> = toml_string.parse()
                .expect(
                    "Failed to parse 'config.template.toml'! \
                    This is a build pipeline issue, report immediately!"
                ); 

            // ^ I really feel like we should expect the template config to be 
            // correct and parse properly. This should generally be caught by CI and 
            // I would consider such a problem more of a "build pipeline issue" rather 
            // than a normal runtime error that should be handled.

            let toml_table = document.as_table();

            self.walk_and_parse_toml_table(
                toml_string,
                None,
                toml_table,
                &mut template_config_items
            );

            memorized_template_config_items = Some(template_config_items);
        }

        ui.memory_mut(
            |mem| mem.data.insert_persisted(template_config_id, memorized_template_config_items)
        );

        // TODO: then parse the actual user's config and make sure it's always up to date
        // 
        // NOTE: ^ I don't think we need to do that any more, but I'll keep the todo until I'm sure.
        // 
        // TODO: (04/09/2025) Remove this when ConfigManager is complete. We don't need to do this 
        // anymore as ConfigManager will handle keeping the config on disk up to date with ram and vice-versa.
    }

    pub fn show_ui(&mut self, ui: &mut Ui, theme: &Theme) {
        self.update(ui);

        let template_config_id = ui.make_persistent_id((self.id, "template_config_items"));
        let memorized_template_config_items = ui.memory_mut(
            |mem| mem.data.get_persisted::<Option<DocItems>>(template_config_id)
        ).unwrap_or_default();

        ui.vertical_centered(|ui| {
            ui.set_max_width(ui.available_width().min(900.0));

            let primary_colour = Color32::from_hex(&theme.pallet.primary.as_hex_string()).unwrap();
            let surface_colour = Color32::from_hex(&theme.pallet.surface.as_hex_string()).unwrap();
            let text_colour = Color32::from_hex(&theme.pallet.text.as_hex_string()).unwrap();

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

                    if let Some(template_config_items) = &memorized_template_config_items {
                        for section in &mut self.sections {
                            let (section_display_info, section_config_key_path) = match section {
                                AnySection::String(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::Bool(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntTiny(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntSmall(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::IntBig(section) => (section.display_info.clone(), section.config_key_path.clone()),
                                AnySection::FloatSmall(section) => (section.display_info.clone(), section.config_key_path.clone()),
                            };

                            let (toml_config_key, config_docstring) = match template_config_items.get(&section_config_key_path) {
                                None => (
                                    &section_config_key_path.split(".").last().expect(
                                        "Failed to split section config key path! This is odd as the \
                                        'config_key_path!()' macro should always return a dotted key path."
                                    ).to_string(),
                                    &section_display_info.description.unwrap_or_default()
                                ),
                                Some((toml_item, docstring)) => {
                                    let key = &toml_item.key;
                                    // let value = &toml_item.value;
    
                                    (key, docstring)
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

    fn walk_and_parse_toml_table(
        &self,
        toml_string: &'a str,
        path: Option<&String>,
        table: &Table,
        template_config_items: &mut DocItems
    ) {
        for (key, item) in table.iter() {
            // println!("Key: {}", key);

            let path = match path {
                Some(path) => &format!("{}.{}", path, key),
                None => &key.to_string(),
            };

            match item {
                Item::None => return,
                Item::Value(value) => {
                    let span_start = item.span().map(|s| s.start).expect(
                        &format!(
                            "Honestly if this blows up, ooooh noooo my badddd :) - Failed to find item for '{}' \
                                in toml document that's needed to get the first column the toml key is mentioned!",
                            path
                        )
                    );

                    let line_number = toml_string[..span_start].chars()
                        .filter(|&char| char == '\n')
                        .count() + 1;

                    let docstring = Self::parse_key_docstring(toml_string, line_number);

                    template_config_items.insert(
                        path.to_string(),
                        (
                            TomlItem {
                                key: key.to_string(),
                                value: value.to_owned()
                            },
                            docstring.to_string()
                        )
                    );
                },
                Item::Table(table) => self.walk_and_parse_toml_table(
                    toml_string,
                    Some(path),
                    table,
                    template_config_items
                ),
                Item::ArrayOfTables(array_of_tables) => {
                    // TODO: this needs testing!

                    for child_table in array_of_tables {
                        self.walk_and_parse_toml_table(
                            toml_string,
                            Some(path),
                            child_table,
                            template_config_items
                        )
                    }
                },
            }
        }
    }

    fn parse_key_docstring(toml_string: &str, key_line_number: usize) -> String {
        let lines: Vec<&str> = toml_string.lines().collect();

        let mut docstring_lines = Vec::new();

        // Start from the line above the key
        let mut index = (key_line_number as isize) - 2; // key_line is 1-based

        while index >= 0 {
            let line = lines[index as usize].trim_start();

            if line.starts_with('#') {
                let formatted_line = line.trim_start_matches('#').trim_start().trim_end();
                docstring_lines.push(formatted_line);
                index -= 1;
            } else {
                // Stop if we hit a non-comment, non-empty line
                break;
            }
        }

        docstring_lines.reverse();
        docstring_lines.join(" ")
    }
}