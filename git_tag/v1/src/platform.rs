#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Platform {
    GitHub,
}

impl Platform {
    // NOTE: not sure if I want to make this public yet
    pub(crate) fn parse_platform_tag(platform_tag_string: &str) -> Option<Self> {
        match platform_tag_string.to_lowercase().trim() {
            "gh" => Some(Platform::GitHub),
            _ => None
        }
    }
}