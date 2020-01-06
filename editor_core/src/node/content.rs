use crate::node::fragment::Fragment;
use crate::node::node::Node;
use std::rc::Rc;

pub enum Content {
    Elements(Fragment),
    Text(String),
    None,
}

use Content::*;
use std::ops::Deref;

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
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match self {
            Elements(ref fragment) => fragment.find_index(offset, round),
            _ => match offset {
                0 => Ok((0, 0)),
                _ => Err(format!("E29380846 {}", offset)),
            }
        }
    }
    pub fn get(&self, index: usize) -> Result<&Rc<Node>, String> {
        match self {
            Elements(ref fragment) => fragment.get(index),
            Text(_) => Err(format!("E76740376 {}", index)),
            None => Err(format!("E39854043 {}", index)),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Elements(ref fragment) => fragment.size(),
            Text(ref text) => text.len(),
            None => 0,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Elements(ref fragment) => fragment.to_string(),
            Text(ref text) => format!("\"{}\"", text),
            None => String::new(),
        }
    }

    pub fn cut(content: &Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        match content.deref() {
            Elements(ref fragment) => {
                let f = fragment.cut(from, to)?;

                Ok(Rc::new(Elements(f)))
            }
            Text(ref text) => match text.get(from..to) {
                Some(slice) => Ok(Rc::new(Text(String::from(slice)))),
                Option::None => Err(format!("E29493677 {} {}", from, to)),
            },
            None => Err(format!("E55556133")),
        }
    }
    pub fn concat(this: &Rc<Self>, other: &Rc<Self>) -> Result<Self, String> {
        match (this.as_ref(), other.as_ref()) {
            (Elements(ref a), Elements(ref b)) => Ok(Elements(Fragment::concat(a, b))),
            (Text(ref a), Text(ref b)) => Ok(Text(format!("{}{}", a, b))),
            (None, None) => Ok(None),
            _ => Err(format!("E75019594")),
        }
    }
    pub fn replace_child(content: &Rc<Self>, index: usize, node: Rc<Node>) -> Result<Rc<Self>, String> {
        match content.deref() {
            Elements(ref fragment) => {
                let n = fragment.get(index)?;

                if Rc::ptr_eq(&n, &node) {
                    Ok(Rc::clone(content))
                } else {
                    Ok(Rc::new(Elements(fragment.replace_child(index, node))))
                }
            }
            Text(_) => Err(format!("E17488085")),
            None => Err(format!("E96098641")),
        }
    }
}
