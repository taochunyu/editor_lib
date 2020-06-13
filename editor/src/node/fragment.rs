use std::rc::Rc;
use crate::node::{TypedNode, Node, cut};
use crate::node::node_type::NodeType;

pub struct Fragment {
    content: Vec<Rc<dyn TypedNode>>,
    size: usize,
}

impl From<Rc<dyn TypedNode>> for Fragment {
    fn from(node: Rc<dyn TypedNode>) -> Self {
        Fragment {
            content: vec![Rc::clone(&node)],
            size: node.size(),
        }
    }
}

impl Fragment {
    fn new(content: Vec<Rc<dyn TypedNode>>) -> Self {
        let size = content.iter().fold(0, |acc, x| acc + x.size());

        Fragment { content, size }
    }

    fn content(&self) -> &Vec<Rc<dyn TypedNode>> {
        &self.content
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> Result<&Rc<dyn TypedNode>, String> {
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

    pub(crate) fn cut(&self, from: usize, to: usize) -> Result<Self, String> {
        if from >= to {
            Ok(Self { content: vec![], size: 0 })
        } else {
            let mut content: Vec<Rc<dyn TypedNode>> = vec![];
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

                    cut(node, cut_from, cut_to)?
                } else {
                    Rc::clone(node)
                };

                content.push(will_push);
                start = end;
            }

            Ok(Fragment { content, size })
        }
    }

    fn replace_child(&self, index: usize, node: Rc<dyn TypedNode>) -> Result<Self, String> {
        match self.content.get(index) {
            None => Err(format!("Index {} outside of fragment", index)),
            Some(child) => {
                let size = self.size + node.size() - child.size();
                let content = self.content.iter().enumerate()
                    .map(|(i, n)| if i == index { node.clone() } else { n.clone() })
                    .collect::<Vec<Rc<dyn TypedNode>>>();

                Ok(Self { content, size })
            }
        }
    }

    fn append(&self, node: Rc<dyn TypedNode>) -> Self {
        let size = self.size + node.size();
        let mut content: Vec<Rc<dyn TypedNode>> = this.content.iter().map(|n| n.clone()).collect();

        if let Some(last) = content.last() {
            if last.
        }
    }
}
