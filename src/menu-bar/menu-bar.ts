import {Component} from "../component.ts";
import {MenuItem} from "../menu-item/menu-item.ts";

/**
 * Manages menu-bars,
 * div -> nav -> ul(this) -> li -> items
 */
export class MenuBar extends Component {
    private constructor() {
        super();
    }

    static override create(parent?: HTMLElement): MenuBar {
        const self = document.createElement<MenuBar>("menu-bar-ul" as keyof HTMLElementTagNameMap);
        if (parent) {
            const div: HTMLElement = document.createElement("div")
            div.className = "MenuBar"

            parent
                .appendChild(div)
                .appendChild(document.createElement("nav"))
                .appendChild(self);
        }
        self.style.height = "2.5rem";
        return self;
    }

    override attachToParent(parent: HTMLElement) {
        const div: HTMLElement = document.createElement("div")
        div.className = "MenuBar"

        parent
            .appendChild(div)
            .appendChild(document.createElement("nav"))
            .appendChild(this);
    }

    addSpacer(width?: string) {
        const spacer = MenuItem.create();
        spacer.style.width = width ?? "10px";
        spacer.className = "Spacer"
        this.appendChild(spacer);
    }
}

customElements.define("menu-bar-ul", MenuBar, { extends: "ul" });
