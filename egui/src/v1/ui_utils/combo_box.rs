use egui::{Response, RichText, Ui};

/// `ui.selectable_value` but the text in the selected value is "extra strong" / pops out more
/// for improved UX.
/// 
/// Please use this in combo boxes instead of `ui.selectable_value` for better UX.
pub fn ui_strong_selectable_value<Value: PartialEq>(
    ui: &mut Ui,
    current_value: &mut Value,
    selected_value: Value,
    text: impl Into<String>
) -> Response {
    let inner_response = ui.scope(|ui| {
        let visuals_mut = ui.visuals_mut();

        // accent colour is too bright most of the time making the text unreadable
        visuals_mut.selection.bg_fill = visuals_mut.selection.bg_fill.gamma_multiply(0.80);

        let mut rich_text = RichText::new(text);

        // the text for selected values is too faint
        if &selected_value == current_value {
            rich_text = rich_text.strong();
        }
    
        ui.selectable_value(
            current_value,
            selected_value,
            rich_text
        )
    });

    inner_response.inner
}