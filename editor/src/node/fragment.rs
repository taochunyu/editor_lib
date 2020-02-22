use crate::node::node::Node;
use std::rc::Rc;
use crate::node::content::Content;
use std::ops::Deref;

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

        Fragment { content, size }
    }
}

impl Clone for Fragment {
    fn clone(&self) -> Self {
        Self {
            content: self.content.iter().map(|n| n.clone()).collect(),
            size: self.size,
        }
    }
}

impl Fragment {
    pub fn content(&self) -> &Vec<Rc<Node>> {
        &self.content
    }
    pub fn cut(&self, from: usize, to: usize) -> Result<Self, String> {
        if from > to {
            Ok(Self {
                content: vec![],
                size: 0,
            })
        } else {
            let mut content: Vec<Rc<Node>> = vec![];
            let mut start: usize = 0;
            let mut end: usize = 0;

            for node in &self.content {
                end += node.size();

                let will_push = if end > from && (start < from || end > to) {
                    let cut_from: usize = if from > start { from - start } else { 0 };
                    let cut_to: usize = if to > node.size() + start {
                        node.size()
                    } else {
                        to - start
                    };
                    let result = Node::cut(node, cut_from, cut_to)?;

                    Rc::clone(&result)
                } else {
                    Rc::clone(node)
                };

                content.push(will_push);
                start += node.size();
            }

            Ok(Self {
                content,
                size: start,
            })
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
                            return Ok((index + 1, end));
                        } else {
                            return Ok((index, cursor));
                        };
                    }

                    cursor = end;
                }

                return Err(format!("Offset {} outside of fragment", offset));
            }
        }
    }
    pub fn get(&self, index: usize) -> Result<&Rc<Node>, String> {
        match self.content.get(index) {
            Some(node) => Ok(node),
            None => Err(format!("Index {} out range of fragment", index)),
        }
    }
    pub fn replace_child(&self, index: usize, node: Rc<Node>) -> Self {
        let size = self.size + node.size() - self.content[index].size();
        let content: Vec<Rc<Node>> = self
            .content
            .iter()
            .enumerate()
            .map(|(i, n)| {
                if i == index {
                    Rc::clone(&node)
                } else {
                    Rc::clone(n)
                }
            })
            .collect();

        Self { content, size }
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn to_string(&self) -> String {
        let content = self.content.iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", content)
    }

    fn append(this: &Self, node: Rc<Node>) -> Self {
        let size = this.size + node.size();
        let mut content: Vec<Rc<Node>> = this.content.iter()
            .map(|n| n.clone())
            .collect();

        if let Some(last) = content.last() {
            if last.is_text() && last.same_markup(&node) {
                let text = Content::concat(&last.content(), &node.content());

                if let Ok(value) = text {
                    content.pop();
                    content.push(Node::with_content(&node, Rc::new(value)));
                }
            }
        } else {
            content.push(node.clone());
        }

        Self {
            content,
            size,
        }
    }

    pub fn concat(this: &Self, other: &Self) -> Self {
        other.content.iter()
            .fold(
                this.clone(),
                |acc, n| Self::append(&acc, n.clone())
            )
    }
}
