use std::rc::Rc;
use crate::node::node::Node;
use crate::position::resolved_position::ResolvedPosition;
use crate::slice::slice::Slice;
use crate::node::content::Content;

pub fn replace(
    from: ResolvedPosition,
    to: ResolvedPosition,
    slice: Slice,
) -> Result<Rc<Node>, String> {
    if slice.open_start() > from.depth() {
        Err(format!("Inserted content deeper than insertion position"))
    } else if slice.open_start() - from.depth() != slice.open_end() - to.depth() {
        Err(format!("Inconsistent open depths"))
    } else {
        replace_outer(from, to, slice, 1)
    }
}

fn replace_outer(
    from: ResolvedPosition,
    to: ResolvedPosition,
    slice: Slice,
    depth: usize,
) -> Result<Rc<Node>, String> {
    let from_index = from.index(depth)?;
    let to_index = to.index(depth)?;
    let node = from.node(depth)?;

    if from_index == to_index && depth < from.depth() - slice.open_start() {
        let inner = replace_outer(from, to, slice, depth + 1)?;
        let content = node.node_content().replace_child(from_index, inner)?;

        Ok(node.with_content(content))
    } else {
        Err(format!("123456"))
    }
}

fn close(node: Rc<Node>, content: Rc<Content>) -> Result<Rc<Node>, String> {
    if !node.node_type().valid_content(&content) {
        Err(format!("Invalid content for node {}", node.node_type().name()))
    } else {
        Ok(node.with_content(content))
    }
}

fn add_node(node: Rc<Node>, target: &mut Vec<Rc<Node>>) {
    match target.last() {
        None => target.push(node),
        Some(last) => {
            if node.is_text() && node.same_markup(last) {
                let content = Content::concat(&last.content(), &node.content());

                if let Ok(c) = content {
                    target.split_last();
                    target.push(node.with_content(Rc::new(c)));
                }
            }
        }
    }
}

fn add_range(
    start: &ResolvedPosition,
    end: &ResolvedPosition,
    depth: usize,
    target: &mut Vec<Rc<Node>>,
) -> Result<(), String> {
    let node = end.node(depth)?;
    let start_index = start.index(depth)?;
    let end_index = end.index(depth)?;

    for index in start_index..end_index {
        let n = node.get(index)?;

        add_node(Rc::clone(n), target);
    }

    Ok(())
}