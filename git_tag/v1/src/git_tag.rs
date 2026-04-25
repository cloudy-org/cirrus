use crate::{error::{Error, Result}, platform::Platform};

#[derive(Debug, PartialEq)]
pub struct GitTag {
    pub owner: String,
    pub repo: Option<String>,
    pub platform: Platform,
}

impl GitTag {
    pub fn get_owner_link(&self) -> String {
        match self.platform {
            Platform::GitHub => format!("https://github.com/{}", self.owner),
        }
    }

    pub fn parse_string(git_tag_string: String) -> Result<Self> {
        let (prefix, platform_tag) = git_tag_string.split_once("@")
            .ok_or_else(|| Error::NoAtSymbol { troubled_string: git_tag_string.clone() })?;

        let platform = Platform::parse_platform_tag(platform_tag)
            .ok_or_else(|| Error::UnknownPlatform { platform: platform_tag.to_string() })?;

        Ok(
            match prefix.trim().split_once("/") {
                Some((owner, repo)) => Self {
                    owner: Self::format_and_reject_illegal_prefix(owner)?,
                    repo: Some(Self::format_and_reject_illegal_prefix(repo)?),
                    platform
                },
                None => Self {
                    owner: Self::format_and_reject_illegal_prefix(prefix)?,
                    repo: None,
                    platform
                },
            }
        )
    }

    fn format_and_reject_illegal_prefix(prefix_string: &str) -> Result<String> {
        let prefix_string = prefix_string.trim().to_string();

        // NOTE: we can increase this when needed but I think 
        // we can also have the limit be git platform dependent.
        if prefix_string.is_empty() || prefix_string.len() > 100 {
            return Err(
                Error::IllegalPrefix {
                    prefix_string,
                    reason: "Prefix is empty or too long (limit: 100 chars).".into()
                }
            );
        }

        let is_allow_string = prefix_string.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');

        match is_allow_string {
            true => Ok(prefix_string),
            false => Err(
                Error::IllegalPrefix {
                    prefix_string,
                    reason: "Prefix must contain only ASCII alphanumeric \
                        characters and underscores ('_') and dashes ('-').".into()
                }
            ),
        }
    }
}