use crate::v1::{error::Result, tiny_lexer::{Token, tokenize}};

pub struct Keys {
    pub(crate) tokens: Vec<Token>
}

impl Keys {
    pub fn new<S: Into<String>>(key_string: S) -> Result<Self> {
        Ok(
            Self {
                tokens: tokenize(key_string.into())?
            }
        )
    }
}