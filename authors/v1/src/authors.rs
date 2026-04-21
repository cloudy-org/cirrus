use crate::{contributor::Contributor, error::{Error, Result}};

#[derive(Default)]
pub struct Authors {
    pub author: Contributor,
    pub contributors: Vec<Contributor>,
}

impl Authors {
    pub fn parse_authors_txt_string(authors_string: &'static str) -> Result<Self> {
        let mut author: Option<Contributor> = None;
        let mut contributors: Vec<Contributor> = Vec::new();

        for (index, line) in authors_string.lines().enumerate() {
            match Contributor::parse_string(&line.to_string()) {
                Ok(contributor) => {
                    if index == 0 {
                        author = Some(contributor);
                        continue;
                    }

                    contributors.push(contributor);
                },
                Err(error) => {
                    if index == 0 {
                        return Err(error);
                    }

                    log::warn!(
                        "Failed to parse contributor on line '{}'! '{}' will be ignored! Error: {}",
                        index, line, error
                    );
                },
            } 
        }

        match author {
            Some(author) => {
                Ok(
                    Self {
                        author,
                        contributors
                    }
                )
            },
            None => Err(Error::NoAuthorFound),
        }
    }
}