use std::{sync::{Arc, RwLock}, time::Duration};

use egui_notify::{Toast, ToastLevel, Toasts};
use egui::{Align2, Color32, Id, Margin, Order, Pos2, RichText, Ui};

use crate::{notifier::{banner::{Banner, BannerPlacement, BannerText}, toast::{ToastError, ToastText}}, scheduler::Scheduler};

#[derive(Clone, Default)]
pub struct Loading {
    pub message: Option<String>,
}

/// A neat way to inform / notify the user of what is going on in the background of your app.
#[derive(Clone)]
pub struct Notifier {
    /// Is anything loading? `None` = nothing is loading.
    pub loading: Option<Loading>,
    pub toasts: Arc<RwLock<Toasts>>,

    banner: Option<Banner>,
    loading_lock: Arc<RwLock<Option<Loading>>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            loading: None,
            toasts: Arc::new(RwLock::new(Toasts::default())),

            banner: None,
            loading_lock: Arc::new(RwLock::new(None)),
        }
    }

    #[deprecated(note = "'toast()' will be deprecated and removed soon, switch to 'show_toast()'!")]
    pub fn toast(&self, text: impl Into<ToastText>, level: ToastLevel, toast_mutator: impl FnOnce(&mut Toast)) {
        self.show_toast(text, level, toast_mutator)
    }

    pub fn show_toast(&self, text: impl Into<ToastText>, level: ToastLevel, toast_mutator: impl FnOnce(&mut Toast)) {
        let text = match text.into() {
            ToastText::Error(ToastError { message, error }) => {
                let log_message = format!(
                    "{} \nDetailed Error:\n\n{}",
                    message.replace("\n\n", "\n"),
                    error
                );

                match level {
                    ToastLevel::Warning => log::warn!("{}", log_message),
                    ToastLevel::Error => log::error!("{}", log_message),
                    _ => log::info!("{}", log_message),
                }

                format!(
                    "{message} \n\nDetailed Error:\n\n{error}"
                )
            },
            ToastText::String(string) => {
                match level {
                    ToastLevel::Warning => log::warn!("{}", string),
                    ToastLevel::Error => log::error!("{}", string),
                    _ => log::info!("{}", string),
                }

                string
            },
        };

        let mut toast = Toast::custom(
            textwrap::wrap(&text, 65).join("\n"),
            level.clone()
        );

        if level == ToastLevel::Error {
            toast.duration(Some(Duration::from_secs(8)));
        }

        if let Ok(mut toasts) = self.toasts.write() {
            toast_mutator(toasts.add(toast));
        }
    }

    /// Displays an overlayer banner at the bottom or the top of the screen.
    pub fn show_banner(&mut self, text: impl Into<BannerText>, placement: BannerPlacement, duration: impl Into<Duration>) {
        let duration = duration.into();

        self.banner = Some(
            Banner {
                text: text.into(),
                placement,
                duration,
                hide_scheduler: Scheduler::new(
                    || {}, duration
                ),
            }
        );
    }

    pub fn set_loading(&self, message: Option<impl ToString>) {
        let message = match message {
            Some(message) => Some(message.to_string()),
            None => None,
        };

        if let Some(msg) = &message {
            log::info!("{}", msg.to_string());
        }

        if let Ok(mut loading) = self.loading_lock.write() {
            *loading = Some(Loading { message });
        }
    }

    pub fn unset_loading(&self) {
        if let Ok(mut loading) = self.loading_lock.write() {
            *loading = None;
        }
    }

    /// Renders toast notifications, overlayer banner and runs update loop for `Notifier.loading`.
    pub fn show(&mut self, ui: &Ui) {
        if let Ok(loading) = self.loading_lock.try_read() {
            self.loading = loading.clone();
        }

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
                        BannerPlacement::TOP => {
                            Pos2::new(window_rect.center().x, window_rect.min.y + 110.0)
                        },
                        BannerPlacement::BOTTOM => {
                            Pos2::new(window_rect.center().x, window_rect.max.y - 138.0)
                        },
                    }
                )
                .pivot(Align2::CENTER_CENTER)
                .interactable(false)
                .order(Order::Foreground)
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

        if let Ok(toasts) = self.toasts.write().as_mut() {
            toasts.show(ui.ctx());
        }
    }
}