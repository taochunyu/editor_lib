import init, { Doc } from '../pkg/web.js';
import { render_editor, update_editor } from "./render.js";
import { get_selection, render_selection } from "./selection.js";

const withTime = (name, fn) => {
    const start = Date.now();
    const res = fn();
    const end = Date.now();

    console.log(`${name}: cost ${end - start} ms`);

    return res;
}

async function run() {
    await init();

    const $editor = document.querySelector(".editor");
    const $cursor = document.querySelector(".cursor");
    const render_cursor = render_selection($editor, $cursor);
    const doc = new Doc();

    let selection = 1;

    // first render
    const root_node = withTime("get doc data", () => doc.get_doc());

    withTime("render doc data", () => render_editor($editor, root_node));
    // withTime("render cursor", () => render_cursor(selection));


    window.addEventListener('click', () => {
        selection = withTime('get selection', () => get_selection($cursor));

        withTime('render selection', () => render_cursor(selection));
    });

    window.addEventListener('keypress', event => {
        const str = event.key;
        const { position, text, size } = withTime("keypress", () => doc.update_doc(selection, selection, str));

        withTime("update editor", () => update_editor($editor, position, text, size));
        selection += str.length;
        withTime("render cursor", () => render_cursor(selection));
    });

    const handle_delete = () => {
        const root_node = doc.update_doc(selection - 1, selection, "");

        update_editor($editor);
        render_editor($editor, root_node);
        selection += 1;
        render_cursor(selection);
    }

    window.addEventListener('keydown', event => {
        switch (event.key) {
            case "Backspace": handle_delete();
                break;
        }
    });
}

run().catch(e => console.log(e));
