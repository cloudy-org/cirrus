use std::{time::Instant, sync::Arc};

use egui::{InnerResponse, Label, Pos2, Response, RichText, Ui, WidgetText};
use sysinfo::{Pid, System};

use crate::{v1::ui_utils::richtext::ui_non_select_label, rich_text_or_unknown};

use crate::v1::error::Error;

pub struct ResourceMonitor {
    system: System,
    pid: Pid,

    cpu_name: RichText,
    total_memory: String,
    started: Option<String>,

    last_cpu_usage: Option<String>,
    last_pull: Instant
}

impl ResourceMonitor {
    pub fn new() -> Result<Self, Error> {
        let system = System::new_all();
        let pid = sysinfo::get_current_pid().map_err(|e| Error::FailedToFindPid(e.into()))?;

        let cpu_name = match system.cpus().first() {
            Some(cpu) => {
                let brand = cpu.brand();
                let name = cpu.name();

                let cleaned_name = Self::clean_cpu_name(format!("{} {}", brand, name));
                RichText::new(cleaned_name)
            }
            None => RichText::new("Unknown").weak()
        };

        let total_memory = re_format::format_bytes(system.total_memory() as f64);

        Ok(
            Self {
                system,
                pid,

                cpu_name,
                total_memory,

                started: None,

                last_cpu_usage: None,
                last_pull: Instant::now()
            }
        )
    }

    fn clean_cpu_name(raw: String) -> String {
        let mut clean = raw.clone();

        let gpu_phrases = [
            "with Radeon Vega Mobile Gfx",
            "with Radeon Vega",
            "with Radeon Graphics",
            "with Radeon",
            "with Vega",
            "Graphics",
            "with Intel",
            "with UHD Graphics",
            "with HD Graphics",
            "Gfx",
            "Graphics Family",
            "Gpu",
            "APU",
            "CPU @",
            "GHz",
            "MHz",

            "cpu0",
        ];

        for phrase in &gpu_phrases {
            clean = clean.replace(phrase, "");
        }

        clean = clean
            .replace("(R)", "")
            .replace("(TM)", "")
            .replace("(tm)", "")
            .replace("  ", " ");

        clean.trim().to_string()
    }

    pub fn show(&mut self, ui: &mut Ui, extra_data: Vec<(String, RichText, Option<String>)>) {
        let window = egui::Window::new(
            WidgetText::RichText(
                Arc::new(RichText::new("💻 Resource Monitor").size(25.0))
            )
        );

        let mut cpu_usage = None;
        let mut memory_usage = None;
        let mut io = None;

        if let Some(process) = self.system.process(self.pid) {
            if self.last_pull.elapsed() >= sysinfo::MINIMUM_CPU_UPDATE_INTERVAL {
                cpu_usage = Some(format!("{}%", process.cpu_usage() / self.system.cpus().len() as f32));

                self.last_pull = Instant::now();
                self.last_cpu_usage = cpu_usage.clone();
            } else {
                cpu_usage = self.last_cpu_usage.clone();
            }

            io = Some(process.disk_usage());

            memory_usage = Some(re_format::format_bytes(process.memory() as f64));
        };

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
                        let cpu_hint = "How much cpu is used by the entire application";
                        let read_hint = "How much was read from your disk by the entire application";
                        let write_hint = "How much was written to your disk by the entire application";

                        ui.label(RichText::new("System info").size(15.0).strong());
                        ui.end_row();

                        ui_non_select_label(ui, "CPU:");
                        ui.label(self.cpu_name.clone());
                        ui.end_row();

                        ui_non_select_label(ui, "Total Memory:");
                        ui.label(&self.total_memory);
                        ui.end_row();

                        ui.separator();
                        ui.end_row();

                        ui.label(RichText::new("App usage").size(15.0).strong());
                        ui.end_row();

                        ui_non_select_label(ui, "App Mem Alloc:")
                            .on_hover_text(mem_allocation_hint);
                        ui.label(
                            rich_text_or_unknown!(memory_usage)
                        );
                        ui.end_row();

                        ui_non_select_label(ui, "CPU Usage:")
                            .on_hover_text(cpu_hint);
                        ui.label(
                            rich_text_or_unknown!(cpu_usage)
                        );
                        ui.end_row();

                        ui_non_select_label(ui, "Total read:")
                            .on_hover_text(read_hint);
                        ui.label(
                            rich_text_or_unknown!(
                                io.map(|f| re_format::format_bytes(f.total_read_bytes as f64))
                            )
                        );
                        ui.end_row();

                        ui_non_select_label(ui, "Total written:")
                            .on_hover_text(write_hint);
                        ui.label(
                            rich_text_or_unknown!(
                                io.map(|f| re_format::format_bytes(f.total_written_bytes as f64))
                            )
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
                        }
                    });
            });
    }
}
