pub struct OverlayerBannerText {
    pub heading: Option<String>,
    pub subtext: Option<String>,
}

impl OverlayerBannerText {
    pub fn new(heading: String, subtext: Option<String>) -> Self {
        Self {
            heading: Some(heading),
            subtext: subtext
        }
    }

    pub fn subtext_only(text: String) -> Self {
        Self {
            heading: None,
            subtext: Some(text)
        }
    }
}