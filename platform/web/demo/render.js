const render_text = $ctx => data => {
    // render self
    const $text = new Text(data.text);

    $text.$$size = data.size;
    $ctx.append($text);
}

const render_paragraph = $ctx => data => {
    // render self
    const $paragraph = document.createElement("p");

    $paragraph.$$size = data.size;
    $ctx.append($paragraph);

    // render children
    const child_render = render($paragraph);

    data.children.forEach(child_render);
}

const render = $ctx => data => {
    switch (data.tag) {
        case "paragraph": return render_paragraph($ctx)(data);
        case "text": return render_text($ctx)(data);
    }
}

export const render_editor = ($editor, data) => {
    const child_render = render($editor);

    data.children.forEach(child_render);
}

export const update_editor = ($editor, index, text, size) => {
    const old_size = $editor.childNodes[index + 1].childNodes[0].$$size;

    $editor.childNodes[index + 1].childNodes[0].textContent = text;
    $editor.childNodes[index + 1].childNodes[0].$$size = size;
    $editor.childNodes[index + 1].$$size += size - old_size;
}
