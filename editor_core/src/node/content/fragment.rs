use crate::node::node::Node;
use std::rc::Rc;

pub struct Fragment {
    content: Vec<Rc<Node>>,
    size: usize,
}

impl From<Rc<Node>> for Fragment {
    fn from(node: Rc<Node>) -> Self {
        Fragment {
            content: vec![Rc::clone(&node)],
            size: node.size(),
        }
    }
}

impl From<Vec<Rc<Node>>> for Fragment {
    fn from(nodes: Vec<Rc<Node>>) -> Self {
        let mut content: Vec<Rc<Node>> = vec![];
        let mut size: usize = 0;

        for node in &nodes {
            content.push(Rc::clone(node));
            size += node.size();
        }

        Fragment {
            content,
            size,
        }
    }
}

impl Fragment {
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match offset {
            0 => Ok((0, 0)),
            d if d == self.size => Ok((self.content.len(), d)),
            d if d > self.size => Err(format!("Offset {} outside of fragment", d)),
            _ => {
                let mut index: usize = 0;
                let mut cursor: usize = 0;

                for item in &self.content {
                    let mut end = offset + item.size();

                    if offset < end {
                        let pos = if round { end } else { cursor };

                        return Ok((index, pos));
                    }

                    index += 1;
                    cursor = end;
                }
            }
        }
    }
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        match self.content.get(index) {
            Some(node) => Ok(Rc::clone(node)),
            None => Err(format!("Index {} out range of fragment", index)),
        }
    }
}
