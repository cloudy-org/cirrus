use egui::{InnerResponse, Ui, UiBuilder};

/// Centers the elements horizontally and virtually by rendering the elements twice.
/// 
/// As you may infer, this is very bad for performance, so AVOID this at all costs 
/// unless the elements being rendered are not too expensive and they must be 
/// resized constantly. 
/// 
/// If you only have to render one element just use 
/// [`egui::Ui::centered_and_justified`] or [`egui::Ui::vertical_centered`] and etc.
pub fn ui_multiple_centered_double_render<R>(ui: &mut Ui, add_contents: impl Fn(&mut Ui) -> R) -> InnerResponse<R> {
    ui.horizontal_centered(|ui| {
        ui.vertical_centered(|ui| {
            let mut hidden_ui = ui.new_child(UiBuilder::new().invisible());

            // We render the elements into an invisible UI first so we can get it's size
            // to then render the elements for real in the real UI but actually centered.
            add_contents(&mut hidden_ui);

            let contents_height = hidden_ui.min_rect().height();

            ui.add_space((ui.max_rect().height() / 2.0) - (contents_height / 2.0));

            add_contents(ui)
        }).inner
    })
}

// #[derive(Clone)]
// struct CenterMultiState {
//     contents_width: Option<f32>,
//     // Represents whether the element needs to be resized (default it to true when constructing your state)
//     sizing_pass: bool
// }

// impl Default for CenterMultiState {
//     fn default() -> Self {
//         Self {
//             contents_width: None,
//             sizing_pass: true
//         }
//     }
// }

// pub struct CenterMulti {}

// impl CenterMulti {
//     pub fn new() -> Self {
//         Self {}
//     }



    // /// This is the best performance.
    // pub fn sizing_pass_show<R>(&mut self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    //     let memory_id = ui.make_persistent_id((ui.id(), "center_multi_state"));

    //     let mut memorized_state = ui.memory_mut(
    //         |mem| mem.data.get_persisted::<CenterMultiState>(memory_id)
    //     ).unwrap_or_default();

    //     ui.vertical_centered(|ui| {
    //         let ui_builder = UiBuilder {
    //             sizing_pass: memorized_state.sizing_pass,
    //             ..Default::default()
    //         };

    //         ui.scope_builder(ui_builder, |ui| {
    //             let is_sizing_pass = ui.is_sizing_pass();

    //             // we only want to apply the spacing to center the widgets when we are not sizing pass.
    //             if !is_sizing_pass {
    //                 ui.add_space(
    //                     (ui.max_rect().width() / 2.) - (memorized_state.contents_width.expect("center multi did not get a sizing pass!") / 2.),
    //                 );
    //             }

    //             let response = add_contents(ui);

    //             if is_sizing_pass {
    //                 memorized_state.contents_width = Some(ui.min_rect().width());
    //                 memorized_state.sizing_pass = false;
    //             }

    //             response
    //         }).inner
    //     })
    // }
// }