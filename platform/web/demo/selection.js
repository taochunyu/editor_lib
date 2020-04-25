export const render_selection = ($editor, $cursor) => selection => {
    let $node = $editor.childNodes[1];
    let offset = selection;

    while (offset > 0) {
        if (offset > $node.$$size) {
            offset -= $node.$$size;
            $node = $node.nextSibling;
        } else if ($node.childNodes.length > 0) {
            offset -= 1;
            $node = $node.childNodes[0];
        } else {
            break;
        }
    }

    const range = document.createRange();

    range.setStart($node, offset);
    range.setEnd($node, offset);

    const editorRect = $editor.getBoundingClientRect();
    const cursorRect = range.getBoundingClientRect();
    const top = cursorRect.top - editorRect.top;
    const left = cursorRect.left - editorRect.left;

    $cursor.style.top = `${top + 1}px`;
    $cursor.style.left = `${left - 1}px`;
}

export const get_selection = $cursor => {
    const native_selection = document.getSelection();
    const path = [];

    let $target = native_selection.focusNode;

    while ($target !== document.body) {
        path.unshift($target);
        $target = $target.parentNode;
    }

    let position = native_selection.focusNode instanceof Text
        ? native_selection.focusOffset + 1
        : 0;

    for (let i = 0; i < path.length; i++) {
        for (let j = 0; j < path[i].childNodes.length; j++) {
            if (path[i].childNodes[j] === $cursor) {
                continue;
            }

            if (path[i + 1] && path[i + 1] !== path[i].childNodes[j]) {
                position += path[i].childNodes[j].$$size;
            } else {
                break;
            }
        }
    }

    return position;
}
