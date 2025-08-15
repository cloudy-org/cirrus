pub struct Section<'a, T> {
    config_key: &'a mut T,
    pub(crate) config_key_path: String,

    pub display_info: SectionDisplayInfo
}

#[derive(Default)]
pub struct SectionDisplayInfo {
    pub name: Option<String>
}

impl<'a, T> Section<'a, T> {
    pub fn new(
        config_key_path: &'a str,
        config_key: &'a mut T,
        display_info: SectionDisplayInfo
    ) -> Self {
        Self {
            config_key,
            config_key_path: Self::strip_and_parse_config_key_path(config_key_path.to_string()),
            display_info,
        }
    }

    fn strip_and_parse_config_key_path(config_key_path: String) -> String {
        let formatted_key_path = config_key_path
            .replace("self.", "");

        let mut split_key_path = formatted_key_path.split(".");

        // we're consuming the root to get rid of the path's prefix ("config.").
        split_key_path.next(); 

        split_key_path.collect::<Vec<&str>>().join(".")
    }
}

pub enum AnySection<'a> {
    String(Section<'a, String>),
    Bool(Section<'a, bool>),
    Int(Section<'a, i64>),
}

impl<'a> From<Section<'a, String>> for AnySection<'a> {
    fn from(section: Section<'a, String>) -> Self {
        AnySection::String(section)
    }
}

impl<'a> From<Section<'a, bool>> for AnySection<'a> {
    fn from(section: Section<'a, bool>) -> Self {
        AnySection::Bool(section)
    }
}

impl<'a> From<Section<'a, i64>> for AnySection<'a> {
    fn from(section: Section<'a, i64>) -> Self {
        AnySection::Int(section)
    }
}