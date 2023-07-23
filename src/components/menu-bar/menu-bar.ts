import {Component, ComponentParent} from "../component.ts";
import {MenuItem} from "../menu-item/menu-item.ts";

/**
 * MenuBar that you can add components to with addItem()
 *
 * menu structure:
 * this -> nav -> ul -> MenuItems
 *
 * MenuItems are list items
 */
export class MenuBar extends Component{
    private readonly _ul: HTMLElement;

    constructor(parent?: ComponentParent) {
        super("div", parent);

        // create nav
        const nav = document.createElement("nav");
        // create unordered list
        this._ul = document.createElement("ul");
        // nav -> ul
        nav.appendChild(this._ul);
        // this -> nav
        this.element.appendChild(nav)
    }

    /**
     * add menu item to menu bar
     * @param item
     */
    addMenuItem(item: MenuItem) {
        this._ul.appendChild(item.element);
    }
}