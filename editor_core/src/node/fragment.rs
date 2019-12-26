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
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        match self.content.get(index) {
            Some(node) => Ok(Rc::clone(node)),
            None => Err(format!("Index {} out range of fragment", index)),
        }
    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match offset {
            0 => Ok((0, 0)),
            d if d == self.size => Ok((self.content.len(), d)),
            d if d > self.size => Err(format!("Offset {} outside of fragment", d)),
            _ => {
                let mut cursor: usize = 0;

                for (index, item) in self.content.iter().enumerate() {
                    let end = cursor + item.size();

                    if offset <= end {
                        if round || end == offset {
                            return Ok((index + 1, end))
                        } else {
                            return Ok((index, cursor))
                        };
                    }

                    cursor = end;
                }

                return Err(format!("Offset {} outside of fragment", offset));
            }
        }
    }
    pub fn replace_child(mut self, index: usize, node: Rc<Node>) -> Self {
        if Rc::ptr_eq(&self.content[index], &node) {
            self
        } else {
            let size = self.size + node.size() - self.content[index].size();

            self.content.splice(index..index, [node].iter().cloned());

            Fragment {
                size,
                content: self.content,
            }
        }
    }
    pub fn size(&self) -> usize {
        self.size
    }
}
