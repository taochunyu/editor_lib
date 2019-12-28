use crate::node::node::Node;
use crate::node::fragment::Fragment;
use std::rc::Rc;

pub enum Content {
    Elements(Fragment),
    Text(String),
    None,
}

use Content::*;

impl From<String> for Content {
    fn from(content: String) -> Self {
        Text(content)
    }
}

impl From<Rc<Node>> for Content {
    fn from(node: Rc<Node>) -> Self {
        Elements(Fragment::from(node))
    }
}

impl From<Vec<Rc<Node>>> for Content {
    fn from(nodes: Vec<Rc<Node>>) -> Self {
        Elements(Fragment::from(nodes))
    }
}

impl Content {
    pub fn get(&self, index: usize) -> Result<&Rc<Node>, String> {
        match self {
            Elements(ref fragment) => fragment.get(index),
            Text(_) => Err(format!("Cannot get child in content text")),
            None => Err(format!("Cannot get child in content none")),
        }
    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match self {
            Elements(ref fragment) => fragment.find_index(offset, round),
            Text(_) => Err(format!("Cannot find index in content text")),
            None => Err(format!("Cannot find index in content none")),
        }
    }
    pub fn replace_child(self: Rc<Self>, index: usize, node: Rc<Node>) -> Result<Rc<Self>, String> {
        match self.as_ref() {
            Elements(ref fragment) => {
                let n = fragment.get(index)?;

                if Rc::ptr_eq(&n, &node) {
                    Ok(Rc::clone(&self))
                } else {
                    Ok(Rc::new(Elements(fragment.replace_child(index, node))))
                }
            },
            Text(_) => Err(format!("Cannot replace child in content text")),
            None => Err(format!("Cannot replace child in content none")),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Elements(ref fragment) => fragment.size(),
            Text(ref text) => text.len(),
            None => 0,
        }
    }

    pub fn concat(this: &Rc<Self>, other: &Rc<Self>) -> Result<Self, String> {
        match (this.as_ref(), other.as_ref()) {
            (Elements(ref a), Elements(ref b)) => {
                Ok(Elements(Fragment::concat(a, b)))
            },
            (Text(ref a), Text(ref b)) => {
                Ok(Text(format!("{}{}", a, b)))
            },
            (None, None) => {
                Ok(None)
            },
            _ => Err(format!("Cannot concat different type node content")),
        }
    }
}
