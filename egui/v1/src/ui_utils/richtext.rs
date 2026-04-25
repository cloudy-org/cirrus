use egui::{Ui, WidgetText, Label, Response};

pub fn ui_non_select_label(ui: &mut Ui, text: impl Into<WidgetText>) -> Response {
    ui.add(Label::new(text).selectable(false))
}

#[macro_export]
macro_rules! rich_text_or_unknown {
    ($opt:expr) => {
        match &$opt {
            Some(string) => RichText::new(string),
            None => RichText::new("Unknown").weak(),
        }
    };

    ($fmt:literal, $opt:expr) => {
        match &$opt {
            Some(string) => RichText::new(format!($fmt, string)),
            None => RichText::new("Unknown").weak(),
        }
    }
}
