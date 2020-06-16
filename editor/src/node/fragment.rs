use std::rc::Rc;
use crate::node::Node;
use crate::node::node_type::NodeType;

pub struct Fragment {
    content: Vec<Rc<dyn Node>>,
    size: usize,
}

impl From<Rc<dyn Node>> for Fragment {
    fn from(node: Rc<dyn Node>) -> Self {
        Fragment {
            content: vec![Rc::clone(&node)],
            size: node.size(),
        }
    }
}

impl From<Vec<Rc<dyn Node>>> for Fragment {
    fn from(nodes: Vec<Rc<dyn Node>>) -> Self {
        let size = nodes.iter().fold(0, |acc, x| acc + x.size());

        Fragment { content: nodes, size }
    }
}

impl Fragment {
    pub(crate) fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> Result<&Rc<dyn Node>, String> {
        match self.content.get(index) {
            Some(node) => Ok(node),
            None => Err(format!("Index {} out range of fragment", index)),
        }
    }

    fn find_index(&self, offset: usize) -> Result<usize, String> {
        match offset {
            0 => Ok(0),
            o if o == self.size => Ok(self.content.len()),
            o if o > self.size => Err(format!("Offset {} outside of fragment.", o)),
            _ => {
                let mut cursor: usize = 0;

                for (index, node) in self.content.iter().enumerate() {
                    let end = cursor + node.size();

                    if offset < end {
                        return Ok(index)
                    }

                    cursor = end;
                }

                Err(format!("Unknown error occurred while finding index in fragment."))
            }
        }
    }

    pub(crate) fn cut(&self, from: usize, to: usize) -> Result<Rc<Self>, String> {
        if from >= to {
            Ok(Rc::new(Self { content: vec![], size: 0 }))
        } else {
            let mut content: Vec<Rc<dyn Node>> = vec![];
            let mut size: usize = 0;
            let mut start: usize = 0;
            let mut end: usize = 0;

            for node in self.content.iter() {
                if start >= to {
                    break;
                }

                end += node.size();

                let will_push = if end > from && (start < from || end > to) {
                    let cut_from: usize = if from > start { from - start } else { 0 };
                    let cut_to: usize = if end > to { to - start } else { node.size() };

                    Node::cut_node(&node, cut_from, cut_to)?
                } else {
                    Rc::clone(node)
                };

                content.push(will_push);
                start = end;
            }

            Ok(Rc::new(Fragment { content, size }))
        }
    }

    fn replace_child(&self, index: usize, node: Rc<dyn Node>) -> Result<Self, String> {
        match self.content.get(index) {
            None => Err(format!("Index {} outside of fragment", index)),
            Some(child) => {
                let size = self.size + node.size() - child.size();
                let content = self.content.iter().enumerate()
                    .map(|(i, n)| if i == index { node.clone() } else { n.clone() })
                    .collect::<Vec<Rc<dyn Node>>>();

                Ok(Self { content: content, size })
            }
        }
    }
}
