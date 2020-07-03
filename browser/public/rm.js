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

export const handleInstruction = instruction => {
    const parsed = JSON.parse(instruction);
    const [type, payload] = parsed;

    if (type === "append") {
        const [id, parentId, name, ...props] = payload;
        const $element = createTag(name, props);


        const $parent = map[parentId];

        map[id] = $element;
        $parent.appendChild($element);
    }
}