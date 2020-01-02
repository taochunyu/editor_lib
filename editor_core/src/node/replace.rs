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
        replace_outer(from, to, slice, 0)
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
        let content = node.content().clone().replace_child(from_index, inner)?;

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

fn check_join(main: &Rc<Node>, sub: &Rc<Node>) -> Result<(), String> {
    if !sub.node_type().compatible_content(main.node_type()) {
        Err(format!("E44654294 {} {}", sub.node_type().name(), main.node_type().name()))
    } else {
        Ok(())
    }
}

fn joinable(
    before: &ResolvedPosition,
    after: &ResolvedPosition,
    depth: usize
) -> Result<Rc<Node>, String> {
    let node_before = before.node(depth)?;
    let node_after = after.node(depth)?;

    check_join(&node_before, &node_after)?;

    Ok(node_before)
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
    node: Rc<Node>,
    start: Option<&ResolvedPosition>,
    end: Option<&ResolvedPosition>,
    depth: usize,
    target: &mut Vec<Rc<Node>>,
) -> Result<(), String> {
    let mut start_index = match start {
        None => 0,
        Some(s) => s.index(depth)?,
    };
    let end_index = match end {
        None => 0,
        Some(e) => e.index(depth)?,
    };

    if let Some(s) = start {
        if s.depth() > depth {
            start_index += 1;
        } else {
            let text_offset = s.text_offset()?;

            if text_offset != 0 {
                let node_after = s.node_after()?;

                if let Some(n) = node_after {
                    add_node(n, target);
                    start_index += 1;
                }
            }
        }
    }

    for index in start_index..end_index {
        let child = node.child(index)?;

        add_node(Rc::clone(child), target);
    }

    if let Some(e) = end {
        let text_offset = e.text_offset()?;

        if e.depth() == depth && text_offset != 0 {
            let node_before = e.node_before()?;

            if let Some(n) = node_before {
                add_node(n, target);
            }
        }
    }

    Ok(())
}

fn replace_two_way(
    from: &ResolvedPosition,
    to: &ResolvedPosition,
    depth: usize
) -> Result<Rc<Content>, String> {
    let mut content: Vec<Rc<Node>> = vec![];
    let node = from.node(depth)?;

    add_range(node, None, Some(from), depth, &mut content)?;

    if from.depth() > depth {
        let node = joinable(from, to, depth + 1)?;
        let result = replace_two_way(from, to, depth + 1)?;
        let closed = close(node, result)?;

        add_node(closed, &mut content);
    }

    Ok(Rc::new(Content::from(content)))
}

fn replace_three_way(
    from: &ResolvedPosition,
    start: &ResolvedPosition,
    end: &ResolvedPosition,
    to: &ResolvedPosition,
    depth: usize,
) -> Result<Rc<Content>, String> {
    let open_start = if from.depth() > depth {
        let node = joinable(from, start, depth + 1)?;

        Some(node)
    } else {
        None
    };

    let open_end = if to.depth() > depth {
        let node = joinable(end, to, depth + 1)?;

        Some(node)
    } else {
        None
    };

    let mut contnet: Vec<Rc<Node>> = vec![];
    let node = from.node(depth)?;

    add_range(node, None, Some(from), depth, &mut contnet)?;

    if open_start.is_some() && open_end.is_some() && start.index(depth) == end.index(depth) {
        check_join(&open_start.unwrap(), &open_end.unwrap())?;

        let res = replace_three_way(from, start, end, to, depth + 1)?;
        let node = close(open_start.unwrap(), res)?;

        add_node(node, &mut content);
    } else {
        if let Some(os) = open_start {
            let res = replace_two_way(from, start, depth + 1)?;
            let node = close(os, res)?;

            add_node(node, &mut content);
        }

        let node = start.node(depth)?;

        add_range(node, Some(start), Some(end), depth + 1, &mut contnet)?;

        if let Some(oe) = open_end {
            let res = replace_two_way(end, to, depth + 1)?;
            let node = close(oe, res)?;

            add_node(node, &mut contnet);
        }
    }

    let node = to.node(depth)?;

    add_range(node, Some(to), None, depth, &mut content)?;

    Ok(Rc::new(Content::from(contnet)))
}
