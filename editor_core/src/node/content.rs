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
    pub fn count(&self) -> usize {
        match self {
            Elements(ref fragment) => fragment.content().len(),
            Text(_) => 1,
            None => 0,
        }
    }
    pub fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        match self.as_ref() {
            Elements(ref fragment) => {
                let f = fragment.cut(from, to)?;

                Ok(Rc::new(Elements(f)))
            },
            Text(ref text) => {
                match text.get(from..to) {
                    Some(slice) => Ok(Rc::new(Text(String::from(slice)))),
                    Option::None => Err(format!("E29493677 {} {}", from, to)),
                }
            },
            None => Err(format!("E55556133"))
        }
    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match self {
            Elements(ref fragment) => fragment.find_index(offset, round),
            Text(_) => Err(format!("E29380846")),
            None => Err(format!("E65932723")),
        }
    }
    pub fn get(&self, index: usize) -> Result<&Rc<Node>, String> {
        match self {
            Elements(ref fragment) => fragment.get(index),
            Text(_) => Err(format!("E76740376")),
            None => Err(format!("E39854043")),
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
            Text(_) => Err(format!("E17488085")),
            None => Err(format!("E96098641")),
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
            _ => Err(format!("E75019594")),
        }
    }
}
