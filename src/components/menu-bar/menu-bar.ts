const template = document.createElement("template");

class MenuBar extends HTMLElement {
    public constructor() {
        super()
        super
            .attachShadow({mode: "open"})
            .appendChild(template.content.cloneNode(true));

        const names = this.querySelectorAll("[data-item-name]");
        const options = this.querySelectorAll("[data-item-options]");
        const menuBar = this.shadowRoot!.firstElementChild;

        for (let i = 0; i < names.length; i++) {
            (names[i] as HTMLElement).style.pointerEvents = "none";

            const dropdown = document.createElement("div");
            menuBar?.appendChild(dropdown)
            dropdown.className = "dropdown";

            const button = document.createElement("button");
            button.className = "dropdown-button";
            dropdown
                .appendChild(button)
                .appendChild(names[i]);

            const menu = document.createElement("div");
            menu.className = "dropdown-menu";
            dropdown
                .appendChild(menu)
                .appendChild(options[i]);
        }
    }

    public connectedCallback() {
        this.shadowRoot!.addEventListener("click", e => {
            const clickedElement = e.target as Element;
            const isDropdown = clickedElement.matches(".dropdown-button");

            // if we clicked menu return
            if (!isDropdown && clickedElement.parentElement?.closest("menu-bar")) {
                return;
            }

            // toggle active of clicked menu item
            let currentDropdown;
            if (isDropdown) {
                currentDropdown = clickedElement.nextElementSibling;
                currentDropdown?.classList.toggle("active");
            }

            // deactivate menu if clicked outside of menu
            for (const dropdown of this.shadowRoot!.querySelectorAll(".dropdown-menu.active")) {
                if (dropdown !== currentDropdown) {
                    dropdown.classList.remove("active");
                }
            }
        });

        document.body.addEventListener("click", e => {
            if (this.contains(e.target as Node | null)) {
                return;
            }

            for (const dropdown of this.shadowRoot!.querySelectorAll(".dropdown-menu.active")) {
                dropdown.classList.remove("active");
            }
        })
    }
}

template.innerHTML =
`
<div class="menu-bar">
</div>

<style>
    .menu-bar {
        display: flex;
        border-bottom: 1px solid var(--hl-color);
        height: 2.5rem;
    }

    .dropdown {
        position: relative;
        z-index: 1;
    }

    .dropdown-button {
        all: unset;
        width: fit-content;
        display: flex;
        height: 2.5rem;
        padding: 0 8px;
    }

    .dropdown-button:hover {
        background: var(--hl-color);
    }
    
    .dropdown-button slot {
        pointer-events: none;
    }
    
    .dropdown-menu {
        position: absolute;
        top: 2.5rem;
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
        transform: translateY(-5px);
        transition: opacity 80ms ease-in-out, transform 80ms ease-in-out;
        pointer-events: none;
    }
    
    .dropdown-menu.active {
        opacity: 1;
        transform: translateY(0);
        pointer-events: auto;
    }
    
</style>
`

customElements.define("menu-bar", MenuBar);
