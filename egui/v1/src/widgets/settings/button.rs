use egui::{Button, CursorIcon, Response, RichText, Ui, Vec2};

pub struct SettingsButton {
    size: Vec2
}

impl SettingsButton {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(2.0, 2.0)
        }
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;

        self
    }

    pub fn show(&self, ui: &mut Ui, show_settings: &mut bool) -> Response {
        let minimal_button_size = ui.spacing().interact_size.y * self.size;

        let settings_button = ui.add(
            Button::new(
                RichText::new("⚙").size(26.0)
            ).min_size(minimal_button_size)
        ).on_hover_cursor(CursorIcon::PointingHand);

        if settings_button.clicked() {
            *show_settings = true;
        }

        settings_button
    }
}