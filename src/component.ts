/**
 * Component interface for web components
 */
export abstract class Component extends HTMLElement {
    private constructor() {
        super();
    }

    // static create?<T extends Component>(...args: any[]): T ;

    attachToParent(parent: HTMLElement) {
        parent?.appendChild(this);
    }
}