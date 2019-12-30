use crate::node::content::Content;
use crate::schema::node_type::NodeType;
use std::rc::Rc;

pub struct Node {
    node_type: Rc<NodeType>,
    content: Rc<Content>,
}

impl Node {
    pub fn content(&self) -> &Rc<Content> {
        &self.content
    }
    pub fn get(&self, index: usize) -> Result<&Rc<Node>, String> {
        self.content.get(index)
    }
    pub fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        if from == 0 && to == self.content.size() {
            Ok(Rc::clone(&self))
        } else {
            let content = self.content.clone().cut(from, to)?;

            Ok(self.with_content(content))
        }
    }
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }
    pub fn node_type(&self) -> Rc<NodeType> {
        Rc::clone(&self.node_type)
    }
    pub fn same_markup(&self, other: &Rc<Node>) -> bool {
        false
    }
    pub fn size(&self) -> usize {
        self.content.size() + self.node_type.border_size()
    }
    pub fn with_content(self: Rc<Self>, content: Rc<Content>) -> Rc<Self> {
        if Rc::ptr_eq(&self.content, &content) {
            Rc::clone(&self)
        } else {
            Rc::new(Self {
                node_type: Rc::clone(&self.node_type),
                content: Rc::clone(&content),
            })
        }
    }

    pub fn new(node_type: Rc<NodeType>, node_content: Rc<Content>) -> Node {
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
