use crate::core::model::fragment::Fragment;
use crate::core::model::node::TreeNode;
use crate::core::model::resolved_position::{resolve_position, ResolvedPosition};
use crate::core::model::slice::Slice;
use std::rc::Rc;

fn close(node: Rc<TreeNode>, content: Option<Rc<Fragment>>) -> Result<Rc<TreeNode>, String> {
    Ok(node.copy(content))
}

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
        Err(String::from("slice size = 0"))
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
                let step_3 = Fragment::append(&step_2, &content.cut(resolved_to.parent_offset, content.size));

                println!("{} {} {}", step_1.size, step_2.size, step_3.size);

                for n in &step_3.content {
                    println!("{}", n.to_string())
                }
                Some(step_3)
            },
            None => None,
        };
        close(parent, content)
    } else {
        Err(String::from("three way"))
    }
}
