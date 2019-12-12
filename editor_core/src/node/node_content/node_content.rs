use crate::node::node::Node;
use crate::node::node_content::fragment::Fragment;
use std::rc::Rc;

pub enum NodeContent {
    Elements(Fragment),
    Text(String),
    None,
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
