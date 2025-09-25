use std::{sync::{Arc, RwLock}, time::Duration};

use cirrus_error::v1::error::CError;
use egui::Context;
use egui_notify::{Toast, ToastLevel, Toasts};

#[derive(Clone, Default)]
pub struct Loading {
    pub message: Option<String>,
}

/// A neat way to inform / notify the user of what is going on in your backend with Egui.
#[derive(Clone)]
pub struct Notifier {
    /// Is anything loading? `None` = nothing is loading.
    pub loading: Option<Loading>,
    pub toasts: Arc<RwLock<Toasts>>,

    loading_lock: Arc<RwLock<Option<Loading>>>,
}

impl Notifier {
    pub fn new() -> Self {
        Self {
            loading: None,
            loading_lock: Arc::new(RwLock::new(None)),
            toasts: Arc::new(RwLock::new(Toasts::default())),
        }
    }

    pub fn update(&mut self, ctx: &Context) {
        if let Ok(loading) = self.loading_lock.try_read() {
            self.loading = loading.clone();
        }

        if let Ok(mut toasts) = self.toasts.write() {
            toasts.show(ctx);
        }
    }

    pub fn toast(&self, text: impl Into<StringOrError>, level: ToastLevel, toast_mutator: impl FnOnce(&mut Toast)) {
        let text = match text.into() {
            StringOrError::Error(error) => {
                let human_message = error.human_message();

                let log_message = match error.actual_error() {
                    Some(actual_error) => format!("{} \nActual Error: {}", human_message.replace("\n\n", "\n"), actual_error),
                    None => human_message.clone(),
                };

                match level {
                    ToastLevel::Warning => log::warn!("{}", log_message),
                    ToastLevel::Error => log::error!("{}", log_message),
                    _ => log::info!("{}", log_message),
                }

                human_message
            },
            StringOrError::String(string) => string,
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

    pub fn set_loading(&self, message: Option<String>) {
        if let Some(msg) = &message {
            log::info!("{}", msg);
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
}

#[derive(Clone)]
pub enum StringOrError {
    Error(Arc<dyn CError>),
    String(String),
}

impl From<String> for StringOrError {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<&str> for StringOrError {
    fn from(string: &str) -> Self {
        Self::String(string.to_owned())
    }
}

impl<E: CError + 'static> From<Box<E>> for StringOrError {
    fn from(error: Box<E>) -> Self {
        Self::Error(Arc::new(*error))
    }
}

impl From<Box<dyn CError>> for StringOrError {
    fn from(error: Box<dyn CError>) -> Self {
        Self::Error(error.into())
    }
}