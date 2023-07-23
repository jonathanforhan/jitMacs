/**
 * menu item class for appending to menu-bar
 */
import {Component, ComponentParent} from "../component.ts";
import {Icon} from "../icon/icon.ts";

class MenuItemText extends Component {
    constructor(text: string, parent?: ComponentParent) {
        super("a", parent);
        this.element.innerHTML = text;
    }
}

/**
 * add* methods fill from left to right
 */
export class MenuItem extends Component {
    constructor(parent?: ComponentParent) {
        super("li", parent);
    }

    addIcon(icon: Icon) {
        this.child = icon;
    }

    addText(text: string) {
        this.child = new MenuItemText(text);
    }
}

/**
 * Used as filler space in menu
 */
export class MenuItemSpacer extends Component {
    constructor(parent?: ComponentParent) {
        super("li", parent);
    }
}
