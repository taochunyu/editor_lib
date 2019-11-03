use crate::model::node::TreeNode;
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
    pub fn text_offset(&self) -> usize {
        self.depth - self.path.last().unwrap().2
    }
    pub fn node_after(&self) -> Option<Rc<TreeNode>> {
        let parent = self.node(self.depth);
        let index = self.index(self.depth);

        if index == parent.child_count() {
            None
        } else {
            Some({
                let text_offset = self.text_offset();
                let child = parent.child(index).unwrap();

                if text_offset != 0 {
                    child.cut(text_offset, child.size())
                } else {
                    child
                }
            })
        }
    }
    pub fn node_before(&self) -> Option<Rc<TreeNode>> {
        let index = self.index(self.depth);

        if index == 0 {
            None
        } else {
            Some({
                let parent = self.node(self.depth);
                let text_offset = self.text_offset();

                if text_offset != 0 {
                    parent.child(index).unwrap().cut(0, text_offset)
                } else {
                    parent.child(index - 1).unwrap()
                }
            })
        }
    }
}

pub fn resolve_position(root: &Rc<TreeNode>, position: usize) -> Result<ResolvedPosition, String> {
    if root.size() < position {
        return Err(format!(
            "resolve_position: Position {} out of Range.",
            position
        ));
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
                        break;
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
