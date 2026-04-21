use cirrus_git_tag::GitTag;

use crate::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Contributor {
    pub name: String,
    pub email: Option<String>,
    pub git_tag: Option<GitTag>,
}

impl Contributor {
    // not sure if I'll make this public yet
    pub(crate) fn parse_string(contributor_string: &String) -> Result<Self> {
        let mut name_end_index = contributor_string.len();

        let git_tag = match contributor_string.find(")") {
            Some(end_index) => {
                let start_index = contributor_string.find("(")
                    .ok_or_else(
                        || Error::IncorrectSyntax {
                            troubled_line: contributor_string.clone(),
                            error: "Missing opening ('(') bracket to enclose git tag.".into()
                        }
                    )?;

                name_end_index = start_index;

                Some(
                    GitTag::parse_string(
                        contributor_string[start_index + 1..end_index].to_string()
                    )?
                )
            },
            None => None,
        };

        let email = match contributor_string.find(">") {
            Some(end_index) => {
                let start_index = contributor_string.find("<")
                    .ok_or_else(
                        || Error::IncorrectSyntax {
                            troubled_line: contributor_string.clone(),
                            error: "Missing opening ('<') arrow bracket to enclose email.".into()
                        }
                    )?;

                name_end_index = start_index;

                Some(contributor_string[start_index + 1..end_index].to_string())
            },
            None => None,
        };

        let name = contributor_string[..name_end_index].trim().to_string();

        Ok(
            Self {
                name,
                email,
                git_tag
            }
        )
    }
}