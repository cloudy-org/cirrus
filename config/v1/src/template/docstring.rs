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
            let formatted_line = line.trim_start_matches('#').trim_start().trim_end();
            docstring_lines.push(formatted_line);
            index -= 1;
        } else {
            // Stop if we hit a non-comment, non-empty line
            break;
        }
    }

    docstring_lines.reverse();

    KeyDocstring {
        description: KeyDocstringDescription {
            short: None, // TODO: implement short (brief) description.
            long: Some(docstring_lines.join(" ")),
        },
    }
}