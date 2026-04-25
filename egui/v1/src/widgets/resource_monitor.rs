use std::{alloc, sync::Arc};

use cap::Cap;
use egui::{Context, Pos2, RichText, Ui, WidgetText, util::History};

use crate::{ui_utils::richtext::ui_non_select_label, rich_text_or_unknown};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

pub struct ResourceMonitor {
    show_settings_ui: bool,
    show_inspection_ui: bool,
    show_texture_ui: bool,
    show_memory_ui: bool,

    frame_times: History<f32>,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        let max_age: f32 = 1.0;
        let max_len = (max_age * 300.0).round() as usize;

        Self {
            show_settings_ui: false,
            show_inspection_ui: false,
            show_texture_ui: false,
            show_memory_ui: false,

            frame_times: History::new(0..max_len, 1.0)
        }
    }

    pub fn show(&mut self, ui: &mut Ui, extra_data: Vec<(String, RichText, Option<String>)>) {
        let ctx = ui.ctx().clone();
        if self.show_inspection_ui {
            egui::Window::new("Inspection UI")
                .min_width(250.0)
                .max_width(400.0)
                .min_height(350.0)
                .max_height(500.0)
                .show(&ctx, |ui| {
                    ctx.inspection_ui(ui);
                });
        }
        if self.show_memory_ui {
            egui::Window::new("Memory UI")
                .min_width(250.0)
                .max_width(400.0)
                .min_height(350.0)
                .max_height(500.0)
                .show(&ctx, |ui| {
                    ctx.memory_ui(ui);
                });
        }
        if self.show_settings_ui {
            egui::Window::new("Settings UI (egui)")
                .min_width(250.0)
                .max_width(400.0)
                .min_height(350.0)
                .max_height(500.0)
                .show(&ctx, |ui| {
                    ctx.settings_ui(ui);
                });
        }
        if self.show_texture_ui {
            egui::Window::new("Texture UI")
                .min_width(250.0)
                .max_width(400.0)
                .min_height(350.0)
                .max_height(500.0)
                .show(&ctx, |ui| {
                    ctx.texture_ui(ui);
                });
        }

        let window = egui::Window::new(
            WidgetText::RichText(
                Arc::new(RichText::new("💻 Resource Monitor").size(25.0))
            )
        );

        window.default_pos(Pos2::new(200.0, 200.0))
            .min_width(150.0)
            .max_width(250.0)
            .min_height(150.0)
            .max_height(250.0)
            .resizable(false)
            .fade_in(false)
            .fade_out(false)
            .scroll([false, true])
            .show(ui.ctx(), |ui| {
                egui::Grid::new("resource_monitor")
                    .min_col_width(ui.available_width() - 25.0)
                    .show(ui, |ui| {
                        let mem_allocation_hint = "How much memory has been allocated to the entire application";
                        let cpu_hint = "Average CPU time spent processing one frame for the entire application";

                        ui.label(RichText::new("App usage").size(15.0).strong());
                        ui.end_row();

                        ui_non_select_label(ui, "App Mem Alloc:")
                            .on_hover_text(mem_allocation_hint);
                        ui.label(
                            re_format::format_bytes(ALLOCATOR.allocated() as f64)
                        );
                        ui.end_row();

                        ui_non_select_label(ui, "Mean CPU Usage:")
                            .on_hover_text(cpu_hint);
                        ui.label(
                            format!("{:.2} ms / frame", 1e3 * self.mean_frame_time())
                        );
                        ui.end_row();

                        if !extra_data.is_empty() {
                            ui.separator();
                            ui.end_row();

                            ui.label(RichText::new("Extra data").size(15.0).strong());
                            ui.end_row();

                            for row in &extra_data {
                                match &row.2 {
                                    Some(string) => ui_non_select_label(ui, &row.0).on_hover_text(string),
                                    None => ui_non_select_label(ui, &row.0)
                                };
                                ui.label(row.1.clone());
                                ui.end_row();
                            }

                            ui.end_row();
                        }

                        ui.separator();
                        ui.end_row();

                        ui.label(RichText::new("egui windows").size(15.0).strong());
                        ui.end_row();

                        for data in vec![
                            ("Settings", &mut self.show_settings_ui),
                            ("Inspection", &mut self.show_inspection_ui),
                            ("Texture", &mut self.show_texture_ui),
                            ("Memory", &mut self.show_memory_ui)
                        ] {
                            ui.checkbox(data.1, data.0);
                            ui.end_row();
                        }
                    });
            });

        ctx.request_repaint();
    }

    fn mean_frame_time(&self) -> f32 {
        self.frame_times.average().unwrap_or_default()
    }

    pub fn on_new_frame(&mut self, ctx: &Context, previous_frame_time: Option<f32>) {
        let now = ctx.input(|i| i.time);
        let previous_frame_time = previous_frame_time.unwrap_or_default();
        if let Some(latest) = self.frame_times.latest_mut() {
            *latest = previous_frame_time;
        }

        self.frame_times.add(now, previous_frame_time);
    }
}
