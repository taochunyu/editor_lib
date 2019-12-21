use crate::node::node::Node;
use std::rc::Rc;

pub struct ResolvedPosition {
    position: usize,
    path: Vec<(Rc<Node>, usize, usize)>,
    parent_offset: usize,
    depth: usize,
}

impl ResolvedPosition {
    pub fn position(&self) -> usize {
        self.position
    }
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn resolve(base: &Rc<Node>, position: usize) -> Result<ResolvedPosition, String> {
        if position > doc.content_size() {
            return Err(format!("Position {} out of range", position));
        }

        let mut path: Vec<(Rc<Node>, usize, usize)> = vec![];
        let mut depth: usize = 0;
        let mut start: usize = 0;
        let mut parent_offset: usize = position;
        let mut cursor: &Rc<Node> = base;

        loop {
            let (index, offset) = cursor.node_content().find_index(parent_offset, false)?;
            let rem = parent_offset - offset;

            path.push((Rc::clone(node), index, start + offset));

            if rem == 0 {
                break;
            }

            cursor = &cursor.child(index)?;

            if cursor.is_text() {
                break;
            }

            parent_offset = rem - 1;
            start += offset + 1;
        }

        Ok(ResolvedPosition {
            depth,
            position,
            path,
            parent_offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::node::node_type::NodeType;

    fn mock_text_node(content: &str) -> Node {
        let node_type = NodeType::new();
        let node_content = NodeContent::
    }

    fn mock_data() -> Node {

    }
}