const template = document.createElement("template");

class MenuBar extends HTMLElement {
    constructor() {
        super()
            .attachShadow({ mode: "open" })
            .appendChild(template.content.cloneNode(true));
    }
}

template.innerHTML =
`
<div class="menu-bar">
  <div class="dropdown">
    <button class="menu-item">
      <slot name="item-name"></slot>
    </button>
    <div class="dropdown-menu">
      <slot></slot>
    </div>
  </div>
</div>

<style>
    .menu-bar {
        border-bottom: 1px solid var(--hl-color);
        height: 2.5rem;
    }

    .dropdown {
        position: relative;
        z-index: 1;
    }

    .menu-item {
        all: unset;
        width: fit-content;
        display: flex;
        height: 2.5rem;
        padding: 0 8px;
    }

    .dropdown > .menu-item:focus,
    .menu-item:hover {
        background: var(--hl-color);
    }

    .icon {
        margin: auto;
    }

    .dropdown-menu {
        position: absolute;
        background: var(--bg-color);
        border: 1px solid var(--hl-color);
        border-radius: 4px;
        padding: 4px;
        margin-left: 2px;
        min-width: 10rem;
        width: fit-content;
        max-width: 30rem;
        box-shadow: 0 2px 5px 0 rgba(0, 0, 0, 0.2);
        opacity: 0;
        transform: translateY(-10px);
        transition: opacity 150ms ease-in-out, transform 150ms ease-in-out;
        pointer-events: none;
    }

    .dropdown > .menu-item:focus + .dropdown-menu {
        opacity: 1;
        transform: translateY(0);
        pointer-events: auto;
    }
</style>
`

customElements.define("menu-bar", MenuBar);
