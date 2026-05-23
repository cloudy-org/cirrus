use std::time::Duration;

use crate::scheduler::Scheduler;

#[derive(Clone)]
pub(super) struct Banner {
    pub(super) text: BannerText,
    pub(super) placement: BannerPlacement,
    pub(super) duration: Duration,
    pub(super) hide_scheduler: Scheduler,
}

#[derive(Hash, Clone)]
pub enum BannerPlacement {
    TOP,
    BOTTOM
}

impl Default for BannerPlacement {
    fn default() -> Self {
        Self::BOTTOM
    }
}

#[derive(Hash, Clone)]
pub struct BannerText {
    pub heading: Option<String>,
    pub subtext: Option<String>,
}

impl BannerText {
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