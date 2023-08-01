import { invoke } from "@tauri-apps/api/tauri";
import * as monaco from "monaco-editor";
import * as fs from "@tauri-apps/api/fs";
import {XTerminal} from "./terminal/terminal.ts";
import {Event, listen} from "@tauri-apps/api/event";
import {InstancePayload} from "./payload.ts";

(async function main() {
    await listen("single-instance", async (e: Event<InstancePayload>) => {
        const contents: string = await fs.readTextFile(e.payload.args[1] || "/home/jon/.bashrc");
        document.getElementById("editor-container")!.childNodes.forEach(node => node.remove());

        monaco.editor.create(document.getElementById("editor-container")!, {
            value: contents,
            language: "javascript",
            automaticLayout: true,
        });

        document.getElementById("editor-container")!.style.display = "block";
        document.getElementById("editor-container")!.style.height = "calc(100vh - 3rem)";
        document.getElementById("terminal-container")!.style.display = "none";
        document.getElementById("terminal-container")!.style.height = "0";
    });

    await XTerminal.create(document.getElementById("terminal-container")!);

    monaco.editor.defineTheme("default", {
        base: "vs-dark",
        inherit: true,
        rules: [],
        colors: {},
    });
    monaco.editor.setTheme("default");
}());

// We must do this to present the main window after the DOM and CSS is loaded or else the screen
// will start with the webview background and flash a different color than the desired theme
window.onload = () => invoke("present");