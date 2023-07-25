import {Component} from "../component.ts";

/**
 * This inherits an Image predefined html element
 * therefor this acts the same as an Image and cannot have
 * Icon method called on it, must be static and take in the icon
 * as a parameter if you would like to mutate
 */
export class Icon extends Component {
    private constructor() {
        super();
    }

    static override create(src: string, parent?: HTMLElement): Icon {
        const icon = document.createElement<Icon>("img" as keyof HTMLElementTagNameMap);
        icon.setAttribute("is", "icon-img")
        icon.src = src;
        parent?.appendChild(this);
        return icon;
    }
}

customElements.define("icon-img", Icon, { extends: "img" })