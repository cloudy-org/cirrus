import { getMatches } from "@tauri-apps/api/cli";

type Themes = "light" | "dark" | "auto";

/**
 * Initializes the window theme to the user's specified theme or else it set's it to the operating system theme.
 */
export function initTheme() {
    getMatches().then((matches) => {
        const dirty_theme = matches.args.theme?.value;

        const theme: Themes = (
            dirty_theme instanceof Array || typeof dirty_theme == "boolean" || dirty_theme == null
        )? "auto": dirty_theme as Themes;

        if (theme == "light" || theme == "dark") {
            localStorage.theme = theme;
        } else if (theme == "auto") {
            if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
                localStorage.theme = "dark";
            } else {
                localStorage.theme = "light";
            }
        }

        console.debug(`Setting '${localStorage.theme}' as the theme...`);

        if (localStorage.theme == "dark") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }
    });
}