use egui::{Color32, CursorIcon, Response, Sense, Stroke, Ui, Vec2};

/// Toggle button from Egui demo's app and modified a little to my liking:
/// https://github.com/emilk/egui/blob/main/crates/egui_demo_lib/src/demo/toggle_switch.rs

pub struct ToggleButton<'a> {
    size: Vec2,
    current_bool_value: &'a mut bool,
}

impl<'a> ToggleButton<'a> {
    pub fn new(current_bool_value: &'a mut bool) -> Self {
        Self {
            size: Vec2::new(2.0, 1.0),
            current_bool_value,
        }
    }

    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;

        self
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let desired_size = ui.spacing().interact_size.y * self.size;

        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            *self.current_bool_value = !*self.current_bool_value;
            response.mark_changed();
        }

        let response = response.on_hover_cursor(CursorIcon::PointingHand);

        response.widget_info(|| {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *self.current_bool_value, "")
        });

        if ui.is_rect_visible(rect) {
            let how_true = ui.ctx().animate_bool_responsive(response.id, *self.current_bool_value);

            let visuals = ui.style().interact_selectable(&response, *self.current_bool_value);

            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();

            ui.painter().rect(
                rect,
                radius,
                visuals.bg_fill.gamma_multiply(0.9),
                visuals.bg_stroke,
                egui::StrokeKind::Inside,
            );

            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_true);
            let center = egui::pos2(circle_x, rect.center().y);

            let circle_bg_fill = Color32::WHITE
                .blend(visuals.bg_fill.gamma_multiply(0.4));

            ui.painter()
                .circle(
                    center,
                    0.75 * radius,
                    circle_bg_fill,
                    Stroke::NONE
                );
        }

        response
    }
}