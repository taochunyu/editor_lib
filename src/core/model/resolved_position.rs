use crate::core::model::node::TreeNode;

pub struct ResolvedPosition {
    pub position: usize,
    pub path: Vec<(usize, usize)>,
    pub parent_offset: usize,
    pub depth: usize,
}

impl ResolvedPosition {
    pub fn index(&self, depth: usize) -> usize {
        self.path[depth].0
    }
}

impl TreeNode {
    pub fn resolve(&self, position: usize) -> Result<ResolvedPosition, String> {
        if self.size() < position {
            return Err(format!("Position {} out of Range.", position));
        }

        let mut path: Vec<(usize, usize)> = vec![];
        let mut cursor: Option<&TreeNode> = Some(self);
        let mut start: usize = 0;
        let mut parent_offset: usize = position;

        while let Some(parent) = cursor {
            match parent.content().content().len() {
                0 => cursor = None,
                _ => {
                    let mut index = 0;
                    let mut offset = 0;

                    for child in parent.content().content() {
                        if offset + child.size() > parent_offset {
                            cursor = Some(child);
                            break;
                        }
                        index += 1;
                        offset += child.size();
                    }

                    path.push((index, start + offset));

                    let rem: usize = parent_offset - offset;

                    if rem == 0 {
                        break;
                    }

                    if cursor.is_some() && cursor.unwrap().content().content().len() == 0 {
                        break;
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
}
