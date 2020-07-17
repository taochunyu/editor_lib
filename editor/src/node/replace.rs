use std::rc::Rc;
use crate::node::Node;
use crate::node::path::{Path, Step};
use crate::node::slice::Slice;
use crate::node::fragment::Fragment;

pub fn replace(base: Rc<dyn Node>, from: usize, to: usize, slice: Slice) -> Result<Rc<dyn Node>, String> {
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

fn replace_outer(from: Rc<Path>, to: Rc<Path>, slice: Slice, depth: usize) -> Result<Rc<dyn Node>, String> {
    let Step { node, index, offset: _ } = from.step(depth)?;

    if index == to.step(depth)?.index && from.depth() > depth + slice.open_start() {
        let child = replace_outer(from, to, slice, depth + 1)?;

        node.replace_child(index, child)
    } else if slice.content().size() == 0 {
        let new_children = replace_two_way(from, to, depth)?;

        node.replace_children(new_children)
    } else if slice.open_start() == 0 && slice.open_end() == 0 && from.clone().depth() == depth && to.depth() == depth {
        let new_children = splice(from, to, slice)?;

        node.replace_children(new_children)
    } else {
        let (start, end) = prepare_slice(slice, from.clone())?;
        let new_children = replace_three_way(from, start, end, to, depth)?;

        node.replace_children(new_children)
    }
}

fn splice(from: Rc<Path>, to: Rc<Path>, slice: Slice) -> Result<Rc<Fragment>, String> {
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

fn with_text_node_split_error(node: Option<Rc<dyn Node>>) -> Result<Rc<dyn Node>, String> {
    match node {
        Some(node) => Ok(node),
        None => Err(String::from("Path should have text node child when text offset != 0.")),
    }
}

fn add_range_before(path:  Rc<Path>, depth: usize, target: &mut Vec<Rc<dyn Node>>) -> Result<(), String>{
    let Step { node, index, offset: _ } = path.step(depth)?;

    for index in 0..index {
        add_node(node.get_child(index)?, target);
    };

    if path.depth() == depth && path.text_offset() != 0 {
        add_node(with_text_node_split_error(path.node_before())?, target);
    }

    Ok(())
}

fn add_range_after(path:  Rc<Path>, depth: usize, target: &mut Vec<Rc<dyn Node>>) -> Result<(), String>{
    let Step { node, index, offset: _ } = path.step(depth)?;
    let mut from = index;
    let to = node.child_count();

    if path.depth() == depth && path.text_offset() != 0 {
        add_node(with_text_node_split_error(path.node_after())?, target);
        from += 1;
    }

    if path.depth() > depth {
        from += 1;
    }

    for index in from..to {
        add_node(node.get_child(index)?, target);
    }

    Ok(())
}

fn add_range(start: Rc<Path>, end: Rc<Path>, depth: usize, target: &mut Vec<Rc<dyn Node>>) -> Result<(), String> {
    let mut from = start.step(depth)?.index;
    let Step { node, index: to, offset: _ } = end.step(depth)?;

    if start.depth() == depth && start.text_offset() != 0 {
        add_node(with_text_node_split_error(start.node_after())?, target);
        from += 1;
    }

    if start.depth() > depth {
        from += 1;
    }

    for index in from..to {
        add_node(node.get_child(index)?, target);
    };

    if end.depth() == depth && end.text_offset() != 0 {
        add_node(with_text_node_split_error(end.node_before())?, target);
    }

    Ok(())
}

fn replace_two_way(from: Rc<Path>, to: Rc<Path>, depth: usize) -> Result<Rc<Fragment>, String> {
    let mut target: Vec<Rc<dyn Node>> = vec![];

    add_range_before(from.clone(), depth, &mut target)?;

    if from.depth() > depth {
        let node = from.step(depth + 1)?.node;
        let content = replace_two_way(from.clone(), to.clone(), depth + 1)?;
        let new_node = node.replace_children(content)?;

        add_node(new_node, &mut target);
    }

    add_range_after(to.clone(), depth, &mut target)?;

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

    add_range_before(from.clone(), depth, &mut target)?;

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

        add_range(start.clone(), end.clone(), depth, &mut target)?;

        if let Some(oe) = open_end {
            let content = replace_two_way(end.clone(), to.clone(), depth + 1)?;
            let node = oe.replace_children(content)?;

            add_node(node, &mut target);
        }
    }

    add_range_after(to.clone(), depth, &mut target)?;

    Ok(Rc::new(Fragment::from(target)))
}

fn prepare_slice(slice: Slice, along: Rc<Path>) -> Result<(Rc<Path>, Rc<Path>), String> {
    let extra = along.depth() - slice.open_start();
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
mod test {
    use crate::test::tools::{create_root, create_empty_slice, create_slice_with_char};
    use crate::node::slice::Slice;

    // root
    //       0   1 2 3 4 5 6    7   8 9 10 11 12 13    14
    // <root> <p> h e l l o </p> <p> w o  r  l  d  </p>  </root>
    fn replace_root(from: usize, to: usize, slice: Slice) {
        let root = create_root();

        println!("{}", root.serialize());

        let root = root.replace(from, to, slice).unwrap();

        println!("{}", root.serialize());
    }

    #[test]
    fn splice() {
        replace_root(3, 4, create_slice_with_char());

        replace_root(6, 8, create_slice_with_char());
    }

    #[test]
    fn replace_two_way() {
        // replace_root(3, 4, create_empty_slice());

        replace_root(6, 8, create_empty_slice());
    }
}
