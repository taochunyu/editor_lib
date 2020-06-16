use crate::node::node_type::NodeType;
use std::rc::Rc;
use crate::node::element_node::ElementNode;

mod node_type;
mod fragment;
mod slice;
mod element_node;
mod text_node;

#[cfg(test)]
mod tests;

pub trait Node {
    fn size(&self) -> usize;
    fn content_size(&self) -> usize;

    fn cut(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String>;
}

impl dyn Node {
    fn cut_node(node: &Rc<dyn Node>, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        if from == 0 && to == node.content_size() {
            Ok(Rc::clone(&node))
        } else {
            Ok(node.cut(from, to)?)
        }
    }
}
