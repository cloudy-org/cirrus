use std::{ops::RangeInclusive};

pub struct Section<'a, T> {
    pub(crate) config_key_path: String,
    pub(crate) config_key_value: &'a mut T,

    pub overrides: SectionOverrides<T>,

    pub display_info: SectionDisplayInfo
}

/// Struct that allows the developer to override and customize 
/// default values and **constraints** that would be set by section widgets.
#[derive(Default)]
pub struct SectionOverrides<T> {
    pub choices: Option<Vec<T>>,
    pub int_range: Option<RangeInclusive<T>>,
    pub text_edit_placeholder: Option<String>
}

#[derive(Default, Clone)]
pub struct SectionDisplayInfo {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl<'a, T> Section<'a, T> {
    pub fn new(
        config_key_path: String,
        config_key_value: &'a mut T,
        overrides: SectionOverrides<T>,
        display_info: SectionDisplayInfo
    ) -> Self {
        Self {
            config_key_path,
            config_key_value,
            overrides,
            display_info,
        }
    }
}