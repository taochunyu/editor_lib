use crate::node::node::Node;
use crate::node::fragment::Fragment;
use std::rc::Rc;

pub enum NodeContent {
    Elements(Fragment),
    Text(String),
    None,
}

impl From<String> for NodeContent {
    fn from(content: String) -> Self {
        NodeContent::Text(content)
    }
}

impl From<Rc<Node>> for NodeContent {
    fn from(node: Rc<Node>) -> Self {
        NodeContent::Elements(Fragment::from(node))
    }
}

impl From<Vec<Rc<Node>>> for NodeContent {
    fn from(nodes: Vec<Rc<Node>>) -> Self {
        NodeContent::Elements(Fragment::from(nodes))
    }
}

impl NodeContent {
    pub fn size(&self) -> usize {
        match self {
            NodeContent::Elements(ref fragment) => fragment.size(),
            NodeContent::Text(ref text) => text.len(),
            NodeContent::None => 0,
        }
    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match self {
            NodeContent::Elements(ref fragment) => fragment.find_index(offset, round),
            NodeContent::Text(_) => Err(format!("Cannot find index in content text")),
            NodeContent::None => Err(format!("Cannot find index in content none")),
        }
    }
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        match self {
            NodeContent::Elements(ref fragment) => fragment.child(index),
            NodeContent::Text(_) => Err(format!("Cannot get child in content text")),
            NodeContent::None => Err(format!("Cannot get child in content none")),
        }
    }
}
