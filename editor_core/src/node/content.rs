use crate::node::node::Node;
use crate::node::fragment::Fragment;
use std::rc::Rc;

pub enum Content {
    Elements(Fragment),
    Text(String),
    None,
}

impl From<String> for Content {
    fn from(content: String) -> Self {
        Content::Text(content)
    }
}

impl From<Rc<Node>> for Content {
    fn from(node: Rc<Node>) -> Self {
        Content::Elements(Fragment::from(node))
    }
}

impl From<Vec<Rc<Node>>> for Content {
    fn from(nodes: Vec<Rc<Node>>) -> Self {
        Content::Elements(Fragment::from(nodes))
    }
}

impl Content {
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        match self {
            Content::Elements(ref fragment) => fragment.child(index),
            Content::Text(_) => Err(format!("Cannot get child in content text")),
            Content::None => Err(format!("Cannot get child in content none")),
        }
    }
    pub fn concat(&self, other: &Rc<Content>) -> Rc<Content> {

    }
    pub fn find_index(&self, offset: usize, round: bool) -> Result<(usize, usize), String> {
        match self {
            Content::Elements(ref fragment) => fragment.find_index(offset, round),
            Content::Text(_) => Err(format!("Cannot find index in content text")),
            Content::None => Err(format!("Cannot find index in content none")),
        }
    }
    pub fn replace_child(&self, index: usize, node: Rc<Node>) -> Result<Rc<Content>, String> {
        match self {
            Content::Elements(ref fragment) => {
                Ok(Rc::new(Content::Elements(fragment.replace_child(index, node))))
            },
            Content::Text(_) => Err(format!("Cannot replace child in content text")),
            Content::None => Err(format!("Cannot replace child in content none")),
        }
    }
    pub fn size(&self) -> usize {
        match self {
            Content::Elements(ref fragment) => fragment.size(),
            Content::Text(ref text) => text.len(),
            Content::None => 0,
        }
    }
}
