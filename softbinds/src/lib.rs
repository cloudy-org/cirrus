use egui::Key;

pub trait SoftbindsExt {
    fn softbinds<S: Into<String>>(keybind_str: S) -> Result<Vec<Key>, String>;
}

fn normalize(str: &str) -> String {
    let lower = str.to_lowercase();
    let trimmed = lower.trim();

    let mut c = trimmed.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl SoftbindsExt for Key {
    fn softbinds<S: Into<String>>(keybind_str: S) -> Result<Vec<Key>, String> {
        let str = normalize(&keybind_str.into());

        let keys: Result<Vec<Key>, String> = if str.contains("+") {
            str.split("+")
                .map(|f| match Key::from_name(&normalize(f.trim())) {
                    Some(key) => Ok(key),
                    None => Err(format!("{} is not a known key", f)),
                })
                .collect()
        } else {
            match Key::from_name(&str) {
                Some(key) => Ok(vec![key]),
                None => Err("Given string is not using a known formatting structure".to_string()),
            }
        };

        Ok(keys?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn softbinds() {
        let test1 = "Tab+T";
        assert_eq!(Key::softbinds(test1).unwrap(), vec![Key::Tab, Key::T]);

        let test2 = "taB+t";
        assert_eq!(Key::softbinds(test2).unwrap(), vec![Key::Tab, Key::T]);

        let test3 = "tAb";
        assert_eq!(Key::softbinds(test3).unwrap(), vec![Key::Tab]);
    }
}
