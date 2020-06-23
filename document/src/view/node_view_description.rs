use std::rc::Rc;

use crate::view::Shared;
use crate::n::node::Node;
use render_vm::element::Element;

pub struct NodeViewDescription {
    parent: Option<Shared<NodeViewDescription>>,
    node: Rc<Node>,
    element: Shared<Element>,
}

impl NodeViewDescription {
}