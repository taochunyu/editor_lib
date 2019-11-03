import init, { bootstrap } from '../pkg/editor_wasm.js';

async function run() {
    await init();

    const $editor = document.createElement("div");

    document.body.appendChild($editor);
    bootstrap($editor);
}

run().catch(console.error);

document.addEventListener("paste", (event) => {
    console.log(event.clipboardData.getData("text/html"));
    console.log('\n');
    console.log(event.clipboardData.getData("text/plain"));
});
