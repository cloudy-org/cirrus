use tauri::Manager;

#[tauri::command]
pub fn init_window(window: tauri::Window) {
    window.get_window("main").unwrap().show().unwrap();
}