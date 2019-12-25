use crate::node::node_content::NodeContent;
use crate::schema::node_type::NodeType;
use std::rc::Rc;

pub struct Node {
    node_type: Rc<NodeType>,
    content: Rc<NodeContent>,
}

impl Node {
    pub fn node_type(&self) -> Rc<NodeType> {
        Rc::clone(&self.node_type)
    }
    pub fn node_content(&self) -> Rc<NodeContent> {
        Rc::clone(&self.content)
    }
    pub fn size(&self) -> usize {
        self.content.size() + self.node_type.border_size()
    }
    pub fn content_size(&self) -> usize {
        self.content.size()
    }
    pub fn child(&self, index: usize) -> Result<Rc<Node>, String> {
        self.content.child(index)
    }
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }

    pub fn new(node_type: Rc<NodeType>, node_content: Rc<NodeContent>) -> Node {
        Node {
            node_type,
            content: node_content,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
