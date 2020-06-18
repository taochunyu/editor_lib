use std::any::Any;
use std::rc::Rc;
use std::ops::Range;
use crate::node::node_type::NodeType;
use crate::node::element_node::ElementNode;
use crate::node::text_node::TextNode;

mod node_type;
mod fragment;
mod slice;
mod element_node;
mod text_node;
mod resolved_offset;

#[cfg(test)]
mod tests;

pub trait Node {
    fn size(&self) -> usize;
    fn content_size(&self) -> usize;
    fn child_count(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String>;
    fn find_index(&self, offset: usize) -> Result<usize, String>;
    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String>;
}

impl dyn Node {
    fn as_text(&self) -> Option<&TextNode> {
        self.as_any().downcast_ref::<TextNode>()
    }

    fn join(&self, node: Rc<dyn Node>) -> Option<Rc<dyn Node>> {
        if let Some(a) = self.as_text() {
            if let Some(b) = node.as_text() {
                return a.try_concat(b)
            }
        }

        None
    }

    fn get_child_range(&self, range: Range<usize>) -> Result<Vec<Rc<dyn Node>>, String> {
        let mut collect: Vec<Rc<dyn Node>> = vec![];

        for index in range.step_by(1) {
            collect.push(self.get_child(index)?)
        }

        Ok(collect)
    }

    fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        if from == 0 && to == self.content_size() {
            Ok(self.clone())
        } else {
            Ok(self.cut_node(from, to)?)
        }
    }
}
