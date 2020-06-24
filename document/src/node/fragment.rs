use std::rc::Rc;
use std::cmp::Ordering;
use crate::node::Node;
use crate::node::element_type::ElementType;

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

    pub(crate) fn count(&self) -> usize {
        self.content.len()
    }

    pub(crate) fn get(&self, index: usize) -> Result<Rc<dyn Node>, String> {
        match self.content.get(index) {
            Some(node) => Ok(node.clone()),
            None => Err(format!("Index {} outside of fragment.", index)),
        }
    }

    pub(crate) fn index(&self, offset: usize) -> Result<usize, String> {
        match offset {
            0 => Ok(0),
            o if o == self.size => Ok(self.content.len()),
            o if o > self.size => Err(format!("Offset {} outside of fragment.", o)),
            _ => {
                let mut window_start: usize = 0;

                for (index, node) in self.content.iter().enumerate() {
                    let window_end = window_start + node.size();

                    if offset <= window_end {
                        return Ok(index);
                    }

                    window_start = window_end;
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

                    node.clone().cut(cut_from, cut_to)?
                } else {
                    Rc::clone(node)
                };

                content.push(will_push);
                start = end;
            }

            Ok(Rc::new(Fragment { content, size }))
        }
    }

    fn replace_child(&self, index: usize, node: Rc<dyn Node>) -> Result<Rc<Self>, String> {
        match self.content.get(index) {
            None => Err(format!("Index {} outside of fragment", index)),
            Some(child) => {
                let size = self.size + node.size() - child.size();
                let content = self.content.iter().enumerate()
                    .map(|(i, n)| if i == index { node.clone() } else { n.clone() })
                    .collect::<Vec<Rc<dyn Node>>>();

                Ok(Rc::new(Self { content, size }))
            }
        }
    }

    fn append(&self, node: Rc<dyn Node>) -> Result<Rc<Self>, String> {
        let size = self.size + node.size();
        let mut content = self.content.iter()
            .map(|node| node.clone())
            .collect::<Vec<Rc<dyn Node>>>();

        if let Some(last) = content.last() {
            if let Some(joined) = last.join(node.clone()) {
                content.pop();
                content.push(joined);

                return Ok(Rc::new(Self { content, size }))
            }
        }

        content.push(node);

        Ok(Rc::new(Self { content, size }))
    }

    fn concat(self: Rc<Self>, fragment: Rc<Fragment>) -> Result<Rc<Self>, String> {
        if let Some((first, rest)) = fragment.content.split_first() {
            if let Some((last, nodes)) = self.content.split_last() {
                let size = self.size + fragment.size;
                let mut content = nodes.iter()
                    .map(|node| node.clone())
                    .collect::<Vec<Rc<dyn Node>>>();

                if let Some(joined) = last.join(first.clone()) {
                    content.push(joined);
                } else {
                    content.push(last.clone());
                    content.push(first.clone());
                }

                rest.iter().for_each(|node| content.push(node.clone()));

                Ok(Rc::new(Self { content, size }))
            } else {
                Ok(fragment.clone())
            }
        } else {
            Ok(self.clone())
        }
    }
}
