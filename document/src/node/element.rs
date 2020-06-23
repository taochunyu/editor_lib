use std::rc::Rc;
use std::any::Any;
use crate::node::node_type::NodeType;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::text::Text;

pub struct Element<T: NodeType> {
    node_type: Rc<T>,
    attributes: Rc<T::Attributes>,
    children: Option<Rc<Fragment>>,
}

impl<T: NodeType> Node for Element<T> {
    fn size(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.size() + 2,
            None => 1,
        }
    }

    fn content_size(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.size(),
            None => 0,
        }
    }

    fn child_count(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.count(),
            None => 0,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        match &self.children {
            Some(fragment) => {
                let result = fragment.cut(from, to)?;

                Ok(Rc::new(Self {
                    node_type: self.node_type.clone(),
                    attributes: self.attributes.clone(),
                    children: Some(result),
                }))
            },
            None => Err(format!("cannot cut node without children")),
        }
    }

    fn index(&self, offset: usize) -> Result<usize, String> {
        match &self.children {
            Some(fragment) => fragment.index(offset),
            None => Err(format!("Cannot find offset index on element node without children.")),
        }
    }

    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String> {
        match &self.children {
            Some(children) => children.get(index),
            None => Err(format!("Cannot get child on element node without children.")),
        }
    }
}

impl<T: NodeType> Element<T> {
    pub(crate) fn new(node_type: Rc<T>, attributes: Rc<T::Attributes>, children: Option<Rc<Fragment>>) -> Rc<Self> {
        Rc::new(Self { node_type, attributes, children })
    }
}
