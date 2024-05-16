import { invoke } from "@tauri-apps/api/tauri";

/**
 * Loads the tauri window.
 */
export function initWindow() {
    invoke("plugin:ctk|init_window");
}