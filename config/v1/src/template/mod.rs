use std::collections::HashMap;

use toml_edit::{Document, Item, Table, TomlError, Value};

use crate::{error::Error, template::docstring::{KeyDocstring, parse_key_docstring}};

mod docstring; // not public as the internals of this crate may change

#[derive(Debug)]
pub struct TemplateKey {
    pub key: String,
    pub docstring: KeyDocstring,
    pub defined_toml_value: Value,
}

pub type TemplateKeys = HashMap<String, TemplateKey>;

pub struct Template<'a> {
    pub keys: Option<TemplateKeys>,

    template_config_toml_string: &'a str,
}

impl<'a> Template<'a> {
    pub fn new(toml_template_string: &'a str) -> Self {
        Self {
            keys: None,
            template_config_toml_string: toml_template_string
        }
    }

    /// Will error if parsing fails, however that really shouldn't be the case if CI tests 
    /// are in place to ensure parsing the template passes. Feel free to `.unwrap()` this.
    pub fn parse_keys(&mut self) -> Result<(), Error> {
        let mut template_keys: TemplateKeys = HashMap::new();

        let toml_string = self.template_config_toml_string;

        let document: Document<String> = toml_string.parse()
            .map_err(
                |error: TomlError| Error::TemplateConfigParseFailure { error: error.to_string() }
            )?;

        let toml_table = document.as_table();

        self.walk_and_parse_toml_table(
            toml_string,
            None,
            toml_table,
            &mut template_keys
        );

        self.keys = Some(template_keys);

        Ok(())
    }

    fn walk_and_parse_toml_table(
        &self,
        toml_string: &'a str,
        path: Option<&String>,
        toml_table: &Table,
        template_keys: &mut TemplateKeys
    ) {
        for (key, item) in toml_table.iter() {
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

                    let docstring = parse_key_docstring(toml_string, line_number);

                    template_keys.insert(
                        path.to_string(),
                        TemplateKey {
                            docstring,
                            key: key.to_string(),
                            defined_toml_value: value.to_owned()
                        }
                    );
                },
                Item::Table(table) => self.walk_and_parse_toml_table(
                    toml_string,
                    Some(path),
                    table,
                    template_keys
                ),
                Item::ArrayOfTables(array_of_tables) => {
                    // TODO: this needs testing!

                    for child_table in array_of_tables {
                        self.walk_and_parse_toml_table(
                            toml_string,
                            Some(path),
                            child_table,
                            template_keys
                        )
                    }
                },
            }
        }
    }
}