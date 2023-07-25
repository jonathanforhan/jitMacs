// import { invoke } from "@tauri-apps/api/tauri";
import {MenuBar} from "./menu-bar/menu-bar.ts";
import {Icon} from "./icon/icon.ts";
import {XTerminal} from "./terminal/terminal.ts";
import {MenuItem} from "./menu-item/menu-item.ts";

const menuBar: MenuBar = MenuBar.create(document.getElementById("root"));

(async function main() {
    const more: Icon = Icon.create("src/assets/icons/vscode-dark/more.svg");
    more.id = "more-menu-icon"
    const moreItem = MenuItem.create();

    const icon = Icon.create("src/assets/icons/vscode-dark/more.svg");
    const iconItem = MenuItem.create();

    menuBar
        .appendChild(moreItem)
        .appendChild(more);
    menuBar
        .addSpacer("4px")

    const term = await XTerminal.create(document.getElementById("root"));
}())