use tauri::{plugin::{Builder, TauriPlugin}, Wry};

pub mod serde;
pub mod commands;

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("ctk")
        .invoke_handler(tauri::generate_handler![commands::init_window])
        .build()
}