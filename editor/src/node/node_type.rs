use std::rc::Rc;
use crate::node::Node;

pub trait NodeType: Sized + 'static {
    type Attributes;

    fn new() -> Rc<Self>;
}


