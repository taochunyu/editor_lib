use std::rc::Rc;
use crate::node::Node;

struct Transform {
    doc: Rc<dyn Node>,
    docs: Vec<Rc<dyn Node>>,
}
