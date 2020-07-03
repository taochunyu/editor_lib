const createTag = (name, props) => {
    switch (name) {
        case "div": return document.createElement("div");
        case "paragraph": return document.createElement("p");
        case "text": return document.createTextNode(props);
    }
}


const $root = document.querySelector("#root");
const map = {
    "0": $root,
};

window.map = map;

export const handleInstruction = instruction => {
    const parsed = JSON.parse(instruction);
    const [type, payload] = parsed;

    if (type === "append") {
        const [id, parentId, name, ...props] = payload;
        const $element = map[id];
        const $parent = map[parentId];

        $parent.appendChild($element);
    }

    if (type === "create") {
        const [id, _, name, ...props] = payload;

        map[id] = createTag(name, props);
    }
}