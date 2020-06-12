const handleRectangular = props => {
    return document.createElement("p");
}

const handleText = props => {
    return new Text(props[0]);
}

const createTag = (name, props) => {
    switch (name) {
        case "rectangular": return handleRectangular(props);
        case "text": return handleText(props);
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