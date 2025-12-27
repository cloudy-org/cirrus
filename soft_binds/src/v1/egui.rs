use egui::Key;

use crate::v1::{error::{Error, Result}, keys::Keys, tiny_lexer::Token};

impl Keys {
    pub fn egui_keys(&self) -> Result<Vec<egui::Key>> {
        let mut keys = Vec::new();

        for token in &self.tokens {
            let key = match token {
                Token::Tab => Key::Tab,
                Token::StrKey(key_string) => {
                    match Key::from_name(key_string) {
                        Some(key) => key,
                        None => return Err(
                            Error::ParseToEguiKeyFailure {
                                key_string: key_string.to_owned()
                            }
                        ),
                    }
                },
                _ => { continue; }
            };

            keys.push(key);
        }

        Ok(keys)
    }

    pub fn egui_modifiers(&self) -> egui::Modifiers {
        let mut modifiers = egui::Modifiers::default();

        for token in &self.tokens {
            // NOTE: if there's any more, we add them here...
            match token {
                Token::Ctrl => { modifiers.ctrl = true; },
                Token::Shift => { modifiers.shift = true; },
                _ => { continue; }
            }
        }

        modifiers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_egui_keys_and_modifiers() -> Result<()> {
        let keys_1 = Keys::new("taB+t")?;
        assert_eq!(keys_1.egui_keys()?, vec![Key::Tab, Key::T]);

        let keys_2 = Keys::new("taB+t")?;
        assert_eq!(keys_2.egui_keys()?, vec![Key::Tab, Key::T]);

        let keys_3 =  Keys::new("tAb")?;
        assert_eq!(keys_3.egui_keys()?, vec![Key::Tab]);

        let keys_4 =  Keys::new("CTRL+R")?;
        assert_eq!(keys_4.egui_keys()?, vec![Key::R]);
        assert!(keys_4.egui_modifiers().ctrl);

        let keys_5 =  Keys::new("SHIFT+A")?;
        assert_eq!(keys_5.egui_keys()?, vec![Key::A]);
        assert!(keys_5.egui_modifiers().shift_only());

        Ok(())
    }
}