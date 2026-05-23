use std::time::Duration;

use egui::{Align2, Color32, Id, Margin, Pos2, RichText, Ui};

use crate::{scheduler::Scheduler, widgets::overlayer_banner::{placement::OverlayerBannerPlacement, text::OverlayerBannerText}};

struct Banner {
    text: OverlayerBannerText,
    placement: OverlayerBannerPlacement,
    duration: Duration,
    hide_scheduler: Scheduler,
}

pub struct OverlayerBanner {
    banner: Option<Banner>,
}

impl OverlayerBanner {
    pub fn new() -> Self {
        Self {
            banner: None
        }
    }

    pub fn show_banner(&mut self, text: OverlayerBannerText, placement: OverlayerBannerPlacement, duration: Duration) {
        self.banner = Some(
            Banner {
                text,
                placement,
                duration,
                hide_scheduler: Scheduler::new(
                    || {}, duration
                ),
            }
        );
    }

    pub fn show(&mut self, ui: &Ui) {
        if let Some(banner) = &mut self.banner {
            let window_rect = ui.ctx().viewport_rect();
            let scheduler = &mut banner.hide_scheduler;

            let duration_progress = scheduler.elapsed().as_secs_f32() / banner.duration.as_secs_f32();

            let alpha = if duration_progress > 0.8 {
                ((1.0 - duration_progress) / 0.1)
                    .clamp(0.0, 1.0)
            } else {
                1.0
            };

            let subtext_colour = Color32::GRAY.gamma_multiply(alpha);
            let background_colour = Color32::from_black_alpha((180.0 * alpha) as u8);
            let heading_text_colour = Color32::from_white_alpha((255.0 * alpha) as u8);

            egui::Area::new(
                Id::new("overlayer_banner_area")
                    .with(&banner.text)
                    .with(&banner.duration)
                    .with(&banner.placement)
            )
                .fixed_pos(
                    match banner.placement {
                        OverlayerBannerPlacement::TOP => {
                            Pos2::new(window_rect.center().x, window_rect.min.y + 110.0)
                        },
                        OverlayerBannerPlacement::BOTTOM => {
                            Pos2::new(window_rect.center().x, window_rect.max.y - 138.0)
                        },
                    }
                )
                .pivot(Align2::CENTER_CENTER)
                .interactable(false)
                .show(ui.ctx(), |ui| {
                    egui::Frame::NONE
                        .fill(background_colour)
                        .corner_radius(8.0)
                        .inner_margin(Margin::symmetric(16, 10))
                        .show(ui, |ui| {
                            if let Some(heading_text) = &banner.text.heading {
                                ui.vertical_centered(|ui| {
                                    ui.colored_label(
                                        heading_text_colour,
                                        RichText::new(heading_text)
                                            .heading()
                                            .size(24.0)
                                    );
                                });
                            }

                            if let Some(subtext_text) = &banner.text.subtext {
                                ui.colored_label(
                                    subtext_colour,
                                    subtext_text
                                );
                            }
                        });
                });

            if scheduler.update().is_some() {
                log::debug!(
                    "'{}' seconds have elapsed, closing overlayer banner...",
                    scheduler.elapsed().as_secs()
                );
                self.banner = None;
                return;
            }

            ui.ctx().request_repaint();
        }
    }
}