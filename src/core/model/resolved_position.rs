use crate::core::model::node::TreeNode;
use std::rc::Rc;

type Path = Vec<(Rc<TreeNode>, usize, usize)>;

pub struct ResolvedPosition {
    pub position: usize,
    pub path: Path,
    pub parent_offset: usize,
    pub depth: usize,
}

impl ResolvedPosition {
    pub fn index(&self, depth: usize) -> usize {
        self.path[depth].1
    }
    pub fn node(&self, depth: usize) -> Rc<TreeNode> {
        Rc::clone(&self.path[depth].0)
    }
}

pub fn resolve_position(root: &Rc<TreeNode>, position: usize) -> Result<ResolvedPosition, String> {
    if root.size() < position {
        return Err(format!("Position {} out of Range.", position));
    }

    let mut path: Path = vec![];
    let mut cursor: Option<&Rc<TreeNode>> = Some(root);
    let mut start: usize = 0;
    let mut parent_offset: usize = position;

    while let Some(parent) = cursor {
        match &parent.content {
            None => cursor = None,
            Some(content) => {
                let mut index = 0;
                let mut offset = 0;

                for child in &content.content {
                    if offset + child.size() > parent_offset {
                        cursor = Some(child);
                        break;
                    }
                    index += 1;
                    offset += child.size();
                }

                path.push((Rc::clone(parent), index, start + offset));

                let rem: usize = parent_offset - offset;

                if rem == 0 {
                    break;
                }

                if let Some(node) = cursor {
                    if node.is_text() {
                        break
                    }
                }

                parent_offset = rem - 1;
                start += offset + 1;
            }
        }
    }

    Ok(ResolvedPosition {
        depth: path.len() - 1,
        position,
        path,
        parent_offset,
    })
}
