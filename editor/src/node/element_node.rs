use std::rc::Rc;
use crate::node::node_type::NodeType;
use crate::node::Node;
use crate::node::fragment::Fragment;

pub struct ElementNode<T: NodeType> {
    node_type: Rc<T>,
    attributes: Rc<T::Attributes>,
    children: Option<Rc<Fragment>>,
}

impl<T: NodeType> Node for ElementNode<T> {
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

    fn cut(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
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
}

impl<T: NodeType> ElementNode<T> {
    fn new(node_type: Rc<T>, attributes: Rc<T::Attributes>, children: Option<Rc<Fragment>>) -> Rc<Self> {
        Rc::new(Self { node_type, attributes, children })
    }
}