use crate::node::content::Content;
use crate::node::node::Node;
use std::rc::Rc;

pub struct NodeType {
    name: String,
    content: String,
}

impl NodeType {
    pub fn border_size(&self) -> usize {
        if self.is_text() { 0 } else if self.content == String::from("") { 1 } else { 2 }
    }
    pub fn create_node(self: Rc<Self>, content: Rc<Content>) -> Node {
        Node::new(Rc::clone(&self), Rc::clone(&content))
    }
    pub fn is_text(&self) -> bool {
        self.name == String::from("text")
    }
    pub fn name(&self) -> String {
        String::from(&self.name)
    }
    pub fn valid_content(&self, content: &Rc<Content>) -> bool {
        true
    }

    pub fn new(name: String, content: String) -> Rc<NodeType> {
        Rc::new(NodeType  {
            name,
            content,
        })
    }
}
