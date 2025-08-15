use std::{collections::HashMap};

use cirrus_theming::v1::Theme;
use egui::{Color32, Context, CornerRadius, Frame, Margin, RichText, Stroke, Ui};
use toml_edit::{Document, Item, Table, Value};

use crate::v1::widgets::settings::section::{AnySection};

pub mod section;

#[derive(Debug)]
struct TomlItem {
    key: String,
    value: Value
}

type Items<'a> = HashMap<String, TomlItem>;
type DocItems<'a> = HashMap<String, (TomlItem, String)>;

pub struct Settings<'a> {
    config_items: Option<Items<'a>>,
    template_config_items: Option<DocItems<'a>>,

    template_config_toml_string: &'a str,

    sections: Vec<AnySection<'a>>
}

impl<'a> Settings<'a> {
    pub fn new(template_config_toml_string: &'a str) -> Self {
        Self {
            config_items: None,
            template_config_items: None,
            template_config_toml_string,

            sections: Vec::new(),
        }
    }

    pub fn add_section<T>(&mut self, section: AnySection<'a>) -> &mut Self {
        self.sections.push(section);

        self
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

                    // TODO: parse toml comment above toml key into this var
                    let docstring = "TODO! (docstring)";

                    // println!("Line num: {:?}", line_number);

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

    fn update(&mut self) {
        // TODO: parse the template config and it's comments ONCE

        if self.template_config_items.is_none() {
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

            self.template_config_items = Some(template_config_items);
        }

        // TODO: then parse the actual user's config and make sure it's always up to date
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, theme: &Theme) {
        self.update();

        let grid_frame_colour = Color32::from_hex(&theme.secondary_colour.hex_code).unwrap();

        ui.vertical_centered(|ui| {
            let grid = Frame::group(&ctx.style())
                .outer_margin(Margin::same(7))
                .stroke(Stroke::NONE)
                .fill(grid_frame_colour)
                .corner_radius(CornerRadius::same(15));

            grid.show(ui, |ui| {
                ui.heading(RichText::new("Settings").strong().size(70.0));
                ui.add_space(5.0);
                ui.end_row();

                // TODO: generate setting fields here deriving from inputted config.template.toml.

                if let Some(template_config_items) = &self.template_config_items {
                    for section in &self.sections {
                        let (display_info, config_key_path) = match section {
                            AnySection::String(section) => (&section.display_info, &section.config_key_path),
                            AnySection::Bool(section) => (&section.display_info, &section.config_key_path),
                            AnySection::Int(section) => (&section.display_info, &section.config_key_path),
                        };

                        if let Some((toml_item, docstring)) = template_config_items.get(config_key_path) {
                            let key = &toml_item.key;
                            let value = &toml_item.value;

                            let config_title = match &display_info.name {
                                Some(name) => name,
                                None => {
                                    &key.to_string()
                                        .replace("_", " ")
                                        .split_whitespace()
                                        .map(|word| {
                                            let mut chars = word.chars();
                                            match chars.next() {
                                                Some(first_char) => format!(
                                                    " {}{}",
                                                    first_char.to_uppercase().collect::<String>(),
                                                    chars.as_str()
                                                ),
                                                None => String::new(),
                                            }
                                        })
                                        .collect()
                                },
                            };

                            Frame::group(ui.style())
                                .fill(grid_frame_colour.gamma_multiply(1.5))
                                .show(ui, |ui|{
                                    ui.heading(config_title);

                                    // TODO: Infer type of input widget in Settings widget when type of provided 
                                    // config variable is established (https://github.com/cloudy-org/roseate/issues/75).
                                    match value {
                                        Value::String(formatted) => todo!(),
                                        Value::Integer(formatted) => todo!(),
                                        Value::Float(formatted) => todo!(),
                                        Value::Boolean(formatted) => {
                                            
                                        },
                                        Value::Datetime(formatted) => todo!(),
                                        Value::Array(array) => todo!(),
                                        Value::InlineTable(inline_table) => todo!(),
                                    }

                                    ui.label(docstring);
                                });
                        }
                    }
                }

                // ui.label("Second row, first column");
                // ui.label("Second row, second column");
                // ui.label("Second row, third column");
                // ui.end_row();
            });
        });

    }

}