use std::rc::Rc;
use crate::node::Node;
use crate::node::content::Content;

pub trait NodeType: Sized + 'static {
    type Attributes;

    fn new() -> Rc<Self>;
}
