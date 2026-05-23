
#[derive(Hash)]
pub enum OverlayerBannerPlacement {
    TOP,
    BOTTOM
}

impl Default for OverlayerBannerPlacement {
    fn default() -> Self {
        Self::BOTTOM
    }
}