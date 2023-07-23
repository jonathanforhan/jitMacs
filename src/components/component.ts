export type ComponentMetaData = {
    className?: string;
    id?: string;
}

/** Component or element id */
export type ComponentParent = Component | string;

/**
 * Abstract Component for building editor elements
 */
export abstract class Component {
    private readonly _element: HTMLElement;

    /**
     * @param type      - the element type i.e. "div" "nav" "li"
     * @param parent    - used to build tree, can be edited later
     * @protected
     *
     * This component className defaults to the child class name but
     * can be changed through the MetaData setter
     */
    protected constructor(type: string, parent?: ComponentParent) {
        this._element = document.createElement(type);
        this._element.className = this.constructor.name;

        if (!parent) return;
        parent instanceof Component
            ? parent.child = this
            : document.getElementById(parent)?.appendChild(this._element);
    }

    get metaData(): ComponentMetaData{
        return {
            className: this._element.className,
            id: this._element.id,
        }
    }

    set metaData(metaData: ComponentMetaData) {
        if (metaData.className) {
            this._element.className = metaData.className;
        }
        if (metaData.id) {
            this._element.id = metaData.id;
        }
    }

    set parent(parent: ComponentParent) {
        if (parent instanceof Component) {
            parent.element.appendChild(this._element);
        } else {
            const _parent = document.getElementById(parent);
            _parent?.appendChild(this._element);
        }
    }

    set child(child: Component) {
        this.element.appendChild(child.element);
    }

    set style(style: Record<string, string>) {
        for (const [key, val] of Object.entries(style)) {
            this._element.style[key as keyof CSSStyleDeclaration] = val;
        }
    }

    get element() {
        return this._element
    }

    get id() {
        return this._element.id;
    }

    set id(id: string) {
        this._element.id = id;
    }
}