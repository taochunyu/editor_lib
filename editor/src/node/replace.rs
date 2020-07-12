use std::rc::Rc;
use crate::node::Node;
use crate::node::path::{Path, Step};
use crate::node::slice::Slice;
use crate::node::fragment::Fragment;

pub fn replace(
    base: Rc<dyn Node>,
    from: usize,
    to: usize,
    slice: Slice
) -> Result<Rc<dyn Node>, String> {
    let resolved_from = base.clone().find_path(from)?;
    let resolved_to = base.clone().find_path(to)?;

    if slice.open_start() > resolved_from.depth() {
        Err(format!("Inserted content deeper than insertion position."))
    } else if slice.open_start() + resolved_to.depth() != slice.open_end() + resolved_from.depth()  {
        Err(format!("Inconsistent open depths."))
    } else {
        replace_outer(resolved_from, resolved_to, slice, 0)
    }
}

fn replace_outer(
    from: Rc<Path>,
    to: Rc<Path>,
    slice: Slice,
    depth: usize,
) -> Result<Rc<dyn Node>, String> {
    let Step { node, index, offset } = from.step(depth)?;

    if index == to.step(depth)?.index && from.depth() > depth + slice.open_start() + 1 {
        let child = replace_outer(from.clone(), to, slice, depth + 1)?;

        node.replace_child(index, child)
    } else if slice.open_start() == 0 && slice.open_end() == 0 && from.clone().depth() == depth && to.depth() == depth {
        let new_children = rebuild(from, to, slice)?;

        node.replace_children(new_children)
    } else if slice.content().size() == 0 {
        let new_children = replace_two_way(from.clone(), to, depth)?;

        node.replace_children(new_children)
    } else {
        let (start, end) = prepare_slice(slice, from.clone())?;
        let new_children = replace_three_way(from.clone(), start, end, to, depth)?;

        node.replace_children(new_children)
    }
}

fn rebuild(from: Rc<Path>, to: Rc<Path>, slice: Slice) -> Result<Rc<Fragment>, String> {
    let parent = from.parent();
    let children = match parent.children() {
        Some(children) => Ok(children),
        None => Err(format!("Cannot replace a leaf node.")),
    }?;

    let from_side = children.cut(0, from.clone().parent_offset())?;
    let to_side = children.cut(to.clone().parent_offset(), parent.content_size())?;

    Ok(from_side.concat(slice.content()).concat(to_side))
}

fn add_node(node: Rc<dyn Node>, target: &mut Vec<Rc<dyn Node>>) {
    if let Some(last) = target.last() {
        if let Some(joined) = last.join(node.clone()) {
            target.pop();
            target.push(joined);

            return;
        }
    }

    target.push(node);
}

fn add_range(
    node: Rc<dyn Node>,
    start: Option<Rc<Path>>,
    end: Option<Rc<Path>>,
    depth: usize,
    target: &mut Vec<Rc<dyn Node>>,
) -> Result<(), String> {
    let mut start_index = match &start {
        Some(path) => path.step(depth)?.index,
        None => 0,
    };
    let end_index = match &end {
        Some(path) => path.step(depth)?.index,
        None => node.child_count(),
    };

    if let Some(path) = &start {
        if path.depth() > depth {
            start_index += 1;
        } else {
            let parent_offset = path.parent_offset();

            if parent_offset != 0 {
                if let Some(node) = path.node_after() {
                    add_node(node, target);

                    start_index += 1
                }
            }
        }
    }

    for index in start_index..end_index {
        add_node(node.get_child(index)?, target);
    }

    if let Some(path) = &end {
        let parent_offset = path.parent_offset();

        if path.depth() == depth && parent_offset != 0 {
            if let Some(node) = path.node_before() {
                add_node(node, target);
            }
        }
    }

    Ok(())
}

fn replace_two_way(from: Rc<Path>, to: Rc<Path>, depth: usize) -> Result<Rc<Fragment>, String> {
    let mut target: Vec<Rc<dyn Node>> = vec![];
    let node = from.step(depth)?.node;

    add_range(node, None, Some(from.clone()), depth, &mut target)?;

    if from.depth() > depth + 1 {
        let node = from.step(depth + 1)?.node;
        let content = replace_two_way(from.clone(), to, depth + 1)?;
        let new_node = node.replace_children(content)?;

        add_node(new_node, &mut target);
    }

    Ok(Rc::new(Fragment::from(target)))
}

fn replace_three_way(
    from: Rc<Path>,
    start: Rc<Path>,
    end: Rc<Path>,
    to: Rc<Path>,
    depth: usize,
) -> Result<Rc<Fragment>, String> {
    let open_start = if from.depth() > depth {
        Some(from.step(depth + 1)?.node)
    } else {
        None
    };
    let open_end = if to.depth() > depth {
        Some(end.step(depth + 1)?.node)
    } else {
        None
    };

    let mut target: Vec<Rc<dyn Node>> = vec![];

    let node = from.step(depth)?.node;

    add_range(node, None, Some(from.clone()), depth, &mut target)?;

    let start_index = start.step(depth)?.index;
    let end_index = end.step(depth)?.index;

    if open_start.is_some() && open_end.is_some() && start_index == end_index {
        let os = open_start.unwrap();
        let content = replace_three_way(from.clone(), start, end, to.clone(), depth + 1)?;
        let node = os.replace_children(content)?;

        add_node(node, &mut target);
    } else {
        if let Some(os) = open_start {
            let content = replace_two_way(from.clone(), start.clone(), depth + 1)?;
            let node = os.replace_children(content)?;

            add_node(node, &mut target);
        }

        let node = start.step(depth)?.node;

        add_range(node, Some(start.clone()), Some(end.clone()), depth, &mut target)?;

        if let Some(oe) = open_end {
            let content = replace_two_way(end.clone(), to.clone(), depth + 1)?;
            let node = oe.replace_children(content)?;

            add_node(node, &mut target);
        }
    }

    let node = to.step(depth)?.node;

    add_range(node, Some(to.clone()), None, depth, &mut target)?;

    Ok(Rc::new(Fragment::from(target)))
}

fn prepare_slice(slice: Slice, along: Rc<Path>) -> Result<(Rc<Path>, Rc<Path>), String> {
    let extra = along.depth() - slice.open_start() - 1;
    let parent = along.step(extra)?.node;

    let mut node = parent.replace_children(slice.content())?;

    for depth in (extra - 1)..=0 {
        node = along.step(depth)?.node.replace_children(Rc::new(Fragment::from(node)))?;
    }

    Ok((
        node.clone().find_path(slice.open_start() + extra)?,
        node.clone().find_path(node.content_size() - slice.open_end())?,
    ))
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::node::Node;
    use crate::node::utils::{create_text, create_element};
    use crate::node_types::paragraph::Paragraph;
    use crate::node_types::root::Root;
    use crate::node::slice::Slice;
    use crate::node::replace::replace;

    fn create_root() -> Rc<dyn Node> {
        let hello = create_text("hello");
        let world = create_text("world");
        let paragraph_hello = create_element::<Paragraph>((), Some(vec![hello]));
        let paragraph_world = create_element::<Paragraph>((), Some(vec![world]));
        let root = create_element::<Root>((), Some(vec![
            paragraph_hello,
            paragraph_world,
        ]));

        root
    }

    fn create_empty_slice() -> Slice {
        Slice::new(0, 0, vec![])
    }

    #[test]
    fn in_line() {
        let root = create_root();
        let slice = create_empty_slice();

        println!("{}", root.serialize());

        let new_root = root.replace(3, 4, slice).unwrap();

        println!("{}", new_root.serialize());
    }
}
