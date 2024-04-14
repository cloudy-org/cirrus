import { platform } from "@tauri-apps/api/os";
import { invoke } from "@tauri-apps/api/tauri";
import { getMatches } from "@tauri-apps/api/cli";

/**
 * Initializes the window theme to the user's specified theme or else it set's it to the operating system theme.
 */
export function initTheme() {
    getMatches().then((matches) => {
        let theme = matches.args.theme?.value;

        if (theme == false || theme == null) {
            theme = "auto";
        }

        platform().then((os) => {
            if (os == "win32") return;

            invoke("plugin:theme|set_theme", {
                theme: theme,
            });
        });
    });
}