#[derive(Debug)]
pub struct KeyDocstring {
    pub description: KeyDocstringDescription
}

#[derive(Debug)]
pub struct KeyDocstringDescription {
    pub short: Option<String>,
    pub long: Option<String>,
}

pub fn parse_key_docstring(toml_string: &str, key_line_number: usize) -> KeyDocstring {
    let lines: Vec<&str> = toml_string.lines().collect();

    let mut docstring_lines = Vec::new();

    // Start from the line above the key
    let mut index = (key_line_number as isize) - 2; // key_line is 1-based

    while index >= 0 {
        let line = lines[index as usize].trim_start();

        if line.starts_with('#') {
            let mut formatted_line = line.trim_start_matches('#').trim_start().trim_end();

            if formatted_line.is_empty() {
                formatted_line = "\n\n";
            }

            docstring_lines.push(formatted_line);
            index -= 1;
        } else {
            // Stop if we hit a non-comment, non-empty line
            break;
        }
    }

    docstring_lines.reverse();

    KeyDocstring {
        description: match docstring_lines.is_empty() {
            true => KeyDocstringDescription {
                short: None,
                long: None,
            },
            false => {
                let constructed_docstring: String = docstring_lines.iter()
                    .map(|line| {
                        match *line {
                            // don't append space if new line
                            "\n\n" => line.to_string(),
                            other_line => format!("{} ", other_line)
                        }
                    })
                    .collect::<String>()
                    .trim_end()
                    .to_owned();

                KeyDocstringDescription {
                    short: None, // TODO: implement short (brief) description.
                    long: Some(constructed_docstring),
                }
            },
        },
    }
}