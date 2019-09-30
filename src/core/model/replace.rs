use crate::core::model::fragment::{Fragment, FragmentSource};
use crate::core::model::node::{Node, TreeNode};
use crate::core::model::resolved_position::{resolve_position, ResolvedPosition};
use crate::core::model::slice::Slice;
use std::rc::Rc;

pub fn replace(
    root: Rc<TreeNode>,
    from: usize,
    to: usize,
    slice: Slice,
) -> Result<Rc<TreeNode>, String> {
    let resolved_from = resolve_position(&root, from)?;
    let resolved_to = resolve_position(&root, to)?;

    if slice.open_start > resolved_from.depth {
        return Err(String::from(
            "Inserted content is deeper than insertion position",
        ));
    }
    if resolved_from.depth - slice.open_start != resolved_to.depth - slice.open_end {
        return Err(String::from("Inconsistent open depths"));
    }

    replace_outer(resolved_from, resolved_to, slice, 0)
}

fn replace_outer(
    resolved_from: ResolvedPosition,
    resolved_to: ResolvedPosition,
    slice: Slice,
    depth: usize,
) -> Result<Rc<TreeNode>, String> {
    let index = resolved_from.index(depth);
    let tree_node = resolved_from.node(depth);

    if index == resolved_to.index(depth) && depth < resolved_from.depth - slice.open_start {
        let inner = replace_outer(resolved_from, resolved_to, slice, depth + 1)?;
        let content = match &tree_node.content {
            Some(content) => Some(content.replace_child(index, inner)),
            None => None,
        };
        Ok(tree_node.copy(content))
    } else if slice.content.size == 0 {
        close(
            tree_node,
            Some(replace_two_way(&resolved_from, &resolved_to, depth)),
        )
    } else if slice.open_start == 0
        && slice.open_end == 0
        && resolved_from.depth == depth
        && resolved_to.depth == depth
    {
        let parent = resolved_from.node(depth);
        let content = match &parent.content {
            Some(content) => {
                let step_1 = content.cut(0, resolved_from.parent_offset);
                let step_2 = Fragment::append(&step_1, &slice.content);

                Some(Fragment::append(
                    &step_2,
                    &content.cut(resolved_to.parent_offset, content.size),
                ))
            }
            None => None,
        };

        close(parent, content)
    } else {
        let (start, end) = prepare_slice_for_replace(slice, &resolved_from);

        close(
            tree_node,
            Some(replace_three_way(
                &resolved_from,
                &start,
                &end,
                &resolved_to,
                depth,
            )),
        )
    }
}

fn close(node: Rc<TreeNode>, content: Option<Rc<Fragment>>) -> Result<Rc<TreeNode>, String> {
    Ok(node.copy(content))
}

fn add_node(node: &Rc<TreeNode>, target: &mut Vec<Rc<TreeNode>>) {
    if let Some(last_child) = target.last() {
        if last_child.need_join(node) {
            let len = target.len();

            target[len - 1] = last_child.join(node);

            return;
        }
    }
    target.push(Rc::clone(node))
}

fn add_range(
    start: Option<&ResolvedPosition>,
    end: Option<&ResolvedPosition>,
    depth: usize,
    target: &mut Vec<Rc<TreeNode>>,
) {
    let mut start_index = 0;
    let node_rp_option = if end.is_some() {
        end
    } else if start.is_some() {
        start
    } else {
        None
    };

    if let Some(node_rp) = node_rp_option {
        let node = node_rp.node(depth);
        let end_index = if let Some(end_rp) = end {
            end_rp.index(depth)
        } else {
            node.size()
        };

        if let Some(start_rp) = start {
            start_index = start_rp.index(depth);

            if start_rp.depth > depth {
                start_index += 1;
            } else if start_rp.text_offset() != 0 {
                add_node(&start_rp.node_after().unwrap(), target);
                start_index += 1;
            }
        }
        for index in start_index..end_index {
            add_node(&node.child(index).unwrap(), target);
        }
        if let Some(end_rp) = end {
            if end_rp.depth == depth && end_rp.text_offset() != 0 {
                add_node(&end_rp.node_before().unwrap(), target);
            }
        }
    }
}

fn check_join(main: &TreeNode, sub: &TreeNode) -> Result<(), String> {
    Ok(())
}

fn joinable(
    before: &ResolvedPosition,
    after: &ResolvedPosition,
    depth: usize,
) -> Result<Rc<TreeNode>, String> {
    let node = before.node(depth);

    check_join(&node, &after.node(depth))?;

    Ok(node)
}

fn replace_two_way(
    resolved_from: &ResolvedPosition,
    resolved_to: &ResolvedPosition,
    depth: usize,
) -> Rc<Fragment> {
    let mut content: Vec<Rc<TreeNode>> = vec![];

    add_range(None, Some(&resolved_from), depth, &mut content);
    if resolved_from.depth > depth {
        let mut children = replace_two_way(&resolved_from, resolved_to, depth + 1);

        add_node(
            &close(resolved_from.node(depth), Some(children)).unwrap(),
            &mut content,
        );
    }
    add_range(Some(resolved_from), None, depth, &mut content);

    let size = content.iter().fold(0, |acc, x| acc + x.size());

    Rc::new(Fragment::new(content, size))
}

fn prepare_slice_for_replace(
    slice: Slice,
    along: &ResolvedPosition,
) -> (ResolvedPosition, ResolvedPosition) {
    let extra = along.depth - slice.open_start;
    let parent = along.node(extra);
    let mut node = parent.copy(Some(slice.content));

    for depth in (extra - 1)..0 {
        node = along
            .node(depth)
            .copy(Some(Rc::new(Fragment::from(FragmentSource::Node(node)))));
    }

    (
        resolve_position(&node, slice.open_start + extra).unwrap(),
        resolve_position(&node, node.content_size() - slice.open_end - extra).unwrap(),
    )
}

fn replace_three_way(
    from: &ResolvedPosition,
    start: &ResolvedPosition,
    end: &ResolvedPosition,
    to: &ResolvedPosition,
    depth: usize,
) -> Rc<Fragment> {
    let open_start = if from.depth > depth {
        Some(joinable(from, start, depth + 1).unwrap())
    } else {
        None
    };
    let open_end = if to.depth > depth {
        Some(joinable(end, to, depth + 1).unwrap())
    } else {
        None
    };
    let mut content: Vec<Rc<TreeNode>> = vec![];

    add_range(None, Some(&from), depth, &mut content);
    if open_start.is_some() && open_end.is_some() && start.index(depth) == end.index(depth) {
        add_node(
            &close(
                open_start.unwrap(),
                Some(replace_three_way(from, start, end, to, depth + 1)),
            )
            .unwrap(),
            &mut content,
        )
    } else {
        if open_start.is_some() {
            add_node(&close(open_start.unwrap(), Some(replace_two_way(from, start, depth + 1))).unwrap(), &mut content);
        }
        add_range(Some(start), Some(end), depth, &mut content);
        if open_end.is_some() {
            add_node(&close(open_end.unwrap(), Some(replace_two_way(end, to, depth + 1))).unwrap(), &mut content);
        }
    }
    add_range(Some(to), None, depth, &mut content);

    let size = content.iter().fold(0, |acc, x| acc + x.size());

    Rc::new(Fragment::new(content, size))
}
