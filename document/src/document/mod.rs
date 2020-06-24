use std::rc::Rc;
use crate::node::Node;

struct Document {
    root: Rc<dyn Node>,
}

impl Document {
    fn new() -> Document {

    }
}