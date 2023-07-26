import {Terminal} from "xterm"
import {FitAddon} from "xterm-addon-fit"
import {Component} from "../component.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";

export class XTerminal extends Component {
    public fd: number;
    private _xterm;

    constructor() {
        super();

        listen("pty-event", () => this.readXTerm()).then();
    }

    static async create(parent?: HTMLElement): Promise<XTerminal> {
        const self = document.createElement<XTerminal>("x-terminal" as keyof HTMLElementTagNameMap);
        self.className = "XTerminal"
        self.id = "x-term"
        parent?.appendChild(self);

        const term = new Terminal({
            fontSize: 18,
            cols: 120,
            rows: 36,
            fontFamily: "monospace"
        });
        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);

        self.fd = await invoke("pty_spawn");
        console.log("front end", self.fd);

        term.open(document.getElementById("x-term"));
        term.onData(async (e: string) => {
            await invoke("pty_write", { fd: self.fd, data: e });
        });

        fitAddon.fit();
        self._xterm = term;
        return self;
    }

    readXTerm() {
        const s = invoke("pty_read", { fd: this.fd }).then((text) => {
            this._xterm.write(text);
        })
    }

    kill() {
        invoke("pty_kill", { fd: this.fd }).then();
    }
}

customElements.define("x-terminal", XTerminal);