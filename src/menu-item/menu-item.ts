import {Component} from "../component.ts";

/**
 * MenuItem, extends li and is the container for
 * items appended to MenuBar
 */
export class MenuItem extends Component {
    static override create(parent?: HTMLElement): MenuItem {
        const self = document.createElement<MenuItem>("li" as keyof HTMLElementTagNameMap);
        self.setAttribute("is", "menu-item-li")
        parent?.appendChild(self);
        return self;
    }
}

customElements.define("menu-item-li", MenuItem, { extends: "li" })