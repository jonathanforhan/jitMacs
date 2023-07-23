import {Component, ComponentParent} from "../component.ts";

/**
 * Icon Component class
 */
export class Icon extends Component {
    /**
     * @param src       - src of icon img
     * @param parent    - Component inheritance
     */
    constructor(src?: string, parent?: ComponentParent) {
        super("img", parent);

        if (src) {
            (this.element as HTMLImageElement).src = src;
        }
    }

    set src(src: string) {
        (this.element as HTMLImageElement).src = src;
    }

    get src() {
        return (this.element as HTMLImageElement).src;
    }
}