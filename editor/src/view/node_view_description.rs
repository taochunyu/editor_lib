use crate::view::Shared;
use crate::n::node::Node;
use std::rc::Rc;
use ui::element::Element;

pub struct NodeViewDescription {
    parent: Option<Shared<NodeView>>,
    node: Rc<Node>,
    element: Shared<Element>,
}

impl NodeViewDescription {
}