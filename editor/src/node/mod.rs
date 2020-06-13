use crate::node::content::Content;
use crate::node::node_type::NodeType;
use std::rc::Rc;

mod node_type;
mod content;
mod fragment;

#[cfg(test)]
mod tests;

pub trait TypedNode {
    fn size(&self) -> usize;
    fn content(&self) -> &Rc<Content>;

    fn replace_content(&self, content: Rc<Content>) -> Rc<dyn TypedNode>;
}

pub struct Node<NT: NodeType> {
    node_type: Rc<NT>,
    attributes: Rc<NT::Attributes>,
    content: Rc<Content>,
}

impl<NT: NodeType> TypedNode for Node<NT> {
    fn size(&self) -> usize {
        self.content.size()
    }
    fn content(&self) -> &Rc<Content> {
        &self.content
    }

    fn replace_content(&self, content: Rc<Content>) -> Rc<dyn TypedNode> {
        Rc::new(Self {
            node_type: self.node_type.clone(),
            attributes: self.attributes.clone(),
            content,
        })
    }
}

impl<NT: NodeType> Node<NT> {
    fn new(node_type: Rc<NT>, attributes: Rc<NT::Attributes>, content: Rc<Content>) -> Rc<Self> {
        Rc::new(Node { node_type, attributes, content })
    }
}

fn replace_content(node: &Rc<dyn TypedNode>, content: Rc<Content>) -> Rc<dyn TypedNode> {
    if Rc::ptr_eq(node.content(), &content) {
        node.clone()
    } else {
        node.replace_content(content)
    }
}

fn cut(node: &Rc<dyn TypedNode>, from: usize, to: usize) -> Result<Rc<dyn TypedNode>, String> {
    if from == 0 && to == 0 {
        Ok(node.clone())
    } else {
        let content = Content::cut(node.content(), from, to)?;

        Ok(replace_content(node, content))
    }
}
