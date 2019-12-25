use crate::node::node_content::NodeContent;
use crate::node::node::Node;
use std::rc::Rc;

pub struct NodeType {
    name: String,
    content: String,
}

impl NodeType {
    pub fn is_text(&self) -> bool {
        self.name == String::from("text")
    }
    pub fn border_size(&self) -> usize {
        if self.is_text() { 0 } else if self.content == String::from("") { 1 } else { 2 }
    }
    pub fn create_node(self: Rc<Self>, content: Rc<NodeContent>) -> Node {
        Node::new(Rc::clone(&self), Rc::clone(&content))
    }

    pub fn new(name: String, content: String) -> Rc<NodeType> {
        Rc::new(NodeType  {
            name,
            content,
        })
    }
}
