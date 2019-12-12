use crate::node::node_content::node_content::NodeContent;
use crate::node::node_type::NodeType;
use std::rc::Rc;

pub struct Node {
    node_type: Rc<NodeType>,
    node_content: Rc<NodeContent>,
}

impl Node {
    pub fn node_type(&self) -> Rc<NodeType> {
        Rc::clone(&self.node_type)
    }
    pub fn node_content(&self) -> Rc<NodeContent> {
        Rc::clone(&self.node_content)
    }
    pub fn size(&self) -> usize {
        self.node_content.size() + self.node_type.border_size()
    }
    pub fn content_size(&self) -> usize {
        self.node_content.size()
    }
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        self.node_content.child(index)
    }
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }
}
