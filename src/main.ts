import { invoke } from "@tauri-apps/api/tauri";
import * as monaco from "monaco-editor";
import * as fs from "@tauri-apps/api/fs";
import {XTerminal} from "./terminal/terminal.ts";

(async function main() {
/*
    await XTerminal.create(document.getElementById("terminal-container")!);

    monaco.editor.defineTheme("default", {
        base: "vs-dark",
        inherit: true,
        rules: [],
        colors: {},
    });
    monaco.editor.setTheme("default");

    const contents = await fs.readTextFile("dev/jitMacs/src/main.ts", { dir: fs.BaseDirectory.Home });
    monaco.editor.create(document.getElementById("editor-container")!, {
        value: contents,
        language: "javascript",
        automaticLayout: true,
    });
*/
}());

// We must do this to present the main window after the DOM and CSS is loaded or else the screen
// will start with the webview background and flash a different color than the desired theme
window.onload = () => invoke("present");