use crate::node::content::Content;
use crate::position::resolved_position::ResolvedPosition;
use crate::schema::node_type::NodeType;
use crate::slice::slice::Slice;
use std::rc::Rc;
use std::ops::Deref;

pub struct Node {
    node_type: Rc<NodeType>,
    content: Rc<Content>,
}

impl Node {
    pub fn child(&self, index: usize) -> Result<&Rc<Self>, String> {
        self.content.get(index)
    }
    pub fn content(&self) -> &Rc<Content> {
        &self.content
    }
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }
    pub fn node_type(&self) -> &Rc<NodeType> {
        &self.node_type
    }
    pub fn same_markup(&self, other: &Rc<Node>) -> bool {
        self.node_type().name() == other.node_type().name()
    }
    pub fn size(&self) -> usize {
        self.content.size() + self.node_type.border_size()
    }
    pub fn to_string(&self) -> String {
        match self.content.deref() {
            Content::Nothing => format!("({}, ,)", self.node_type.name()),
            _ => format!("({}, ,{})", self.node_type.name(), self.content.to_string()),
        }
    }

    pub fn cut(node: &Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        if from == 0 && to == node.content.size() {
            Ok(Rc::clone(&node))
        } else {
            let content = Content::cut(node.content(), from, to)?;

            Ok(Node::with_content(&node, content))
        }
    }
    pub fn new(node_type: Rc<NodeType>, node_content: Rc<Content>) -> Node {
        Node {
            node_type,
            content: node_content,
        }
    }
    pub fn with_content(node: &Rc<Self>, content: Rc<Content>) -> Rc<Self> {
        if Rc::ptr_eq(node.content(), &content) {
            Rc::clone(node)
        } else {
            Rc::new(Self {
                node_type: Rc::clone(node.node_type()),
                content: Rc::clone(&content),
            })
        }
    }
}

#[cfg(test)]
pub mod tests {
}
