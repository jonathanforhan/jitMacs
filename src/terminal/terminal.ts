import {Terminal} from "xterm"
import {FitAddon} from "xterm-addon-fit"
import {invoke} from "@tauri-apps/api/tauri";
import {Event, listen} from "@tauri-apps/api/event";
import {PtyPayload} from "../payload.ts";

export type WindowSize = {
    numRows: number,
    numCols: number,
    cellWidth: number,
    cellHeight: number
}

export class XTerminal extends HTMLElement {
    public fd: number = -1;
    private _xterm: Terminal | undefined;

    public constructor() {
        super();

        listen("pty-event", (e: Event<PtyPayload>) => {
            console.assert(e.payload.status === 200);
            if (e.payload.fd === this.fd) {
                this._xterm?.write(e.payload.res);
            }
        }).then();

        // TODO handle death
        listen("pty-die", (e: Event<PtyPayload>) => {
            console.assert(e.payload.status === 200);
            console.log(e.payload.fd, " is dead");
        }).then();
    }

    public static async create(parent?: HTMLElement): Promise<XTerminal> {
        const self: XTerminal = <XTerminal>document.createElement("x-terminal" as keyof HTMLElementTagNameMap);
        self.className = "XTerminal"
        self.id = "x-term"
        parent?.appendChild(self);

        self._xterm = new Terminal({
            fontSize: 14,
            fontFamily: "monospace",
            theme: {
                background: "#232225"
            }
        });
        const fitAddon = new FitAddon();
        self._xterm.loadAddon(fitAddon);

        self.fd = await invoke("pty_spawn");
        self._xterm.open(document.getElementById("x-term")!);

        self._xterm.onData(async (e: string) => {
            await invoke("pty_write", { fd: self.fd, data: e });
        });
        new ResizeObserver(async () => {
            fitAddon.fit()
            let dimensions = fitAddon.proposeDimensions() || { cols: 173, rows: 48 }
            await invoke("pty_resize", {
                fd: self.fd, windowSize: {
                    numRows: dimensions.rows,
                    numCols: dimensions.cols,
                    cellWidth: 10,
                    cellHeight: 20
                }
            })
        }).observe(self as Element);

        return self;
    }

    kill() {
        invoke("pty_kill", { fd: this.fd }).then();
    }
}

customElements.define("x-terminal", XTerminal);