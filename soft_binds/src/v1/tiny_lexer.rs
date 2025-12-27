use crate::v1::error::{Error, Result};

// NOTE: For now, all chars will be generalized into 
// 'StrKey' while a select few like "+" will be tokenized.
// 
// In the future more keys will be tokenised manually by our lexer.
#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,

    Ctrl,
    Tab,
    Shift,

    StrKey(String),
}

// NOTE: the tokenizer is more advanced than it should be to 
// leave room in the future for more advanced levels of parsing.
pub fn tokenize(key_binds: String) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    let mut chars = key_binds.chars().peekable();

    let mut position: u32 = 0;

    while let Some(char) = chars.next() {
        position += 1;

        match char {
            char if char.is_whitespace() => continue,
            '+' => tokens.push(Token::Plus),
            'a'..='z' | 'A'..='Z' | '0'..='9' => {
                let mut key = char.to_string();

                while let Some(&next_char) = chars.peek() {
                    if next_char.is_ascii_alphanumeric() {
                        key.push(chars.next().unwrap());

                        position += 1;
                    } else {
                        break;
                    }
                }

                match key.to_lowercase().as_str() {
                    "ctrl" => tokens.push(Token::Ctrl),
                    "tab" => tokens.push(Token::Tab),
                    "shift" => tokens.push(Token::Shift),
                    _ => tokens.push(Token::StrKey(key.to_uppercase())),
                }
            },
            unknown_char => return Err(
                Error::UnknownKeyBindChar { char: unknown_char, position }
            )
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenisation() -> Result<()> {
        assert_eq!(
            tokenize("Tab+T".into())?,
            vec![Token::Tab, Token::Plus, Token::StrKey("T".into())]
        );

        assert_eq!(
            tokenize("taB+t".into())?,
            vec![Token::Tab, Token::Plus, Token::StrKey("T".into())]
        );

        assert_eq!(tokenize("tAb".into())?, vec![Token::Tab]);

        assert_eq!(tokenize("shift+".into())?, vec![Token::Shift, Token::Plus]);

        assert!(tokenize("shift-".into()).is_err());
        assert_eq!(tokenize("".into())?, vec![]);
        assert_eq!(tokenize(" ".into())?, vec![]);

        assert_eq!(
            tokenize("ctrl + a".into())?,
            vec![Token::Ctrl, Token::Plus, Token::StrKey("A".into())]
        );

        Ok(())
    }
}