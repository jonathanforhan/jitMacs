// import { invoke } from "@tauri-apps/api/tauri";
import {MenuBar} from "./components/menu-bar/menu-bar.ts";
import {MenuItem, MenuItemSpacer} from "./components/menu-item/menu-item.ts";
import {Icon} from "./components/icon/icon.ts";

function initMenu() {
    const menuBar = new MenuBar("main");

    const more = new MenuItem(menuBar);
    more.addIcon(new Icon("src/assets/icons/vscode-dark/more.svg"))

    new MenuItemSpacer(menuBar);

    const project = new MenuItem(menuBar);
    project.addText("project")
    const projArrow = new Icon("src/assets/icons/vscode-dark/chevron-down.svg");
    projArrow.style = { "margin-left": "4px" }
    project.addIcon(projArrow);

    const versionControl = new MenuItem(menuBar);
    const vcIcon = new Icon("src/assets/icons/vscode-dark/source-control.svg");
    vcIcon.style = { height: "50%" }
    vcIcon.style = { "margin-right": "4px" }
    versionControl.addIcon(vcIcon);
    versionControl.addText("branch")
    const vcArrow = new Icon("src/assets/icons/vscode-dark/chevron-down.svg");
    vcArrow.style = { "margin-left": "4px" }
    versionControl.addIcon(vcArrow);
}

(function main() {
    initMenu();
}())