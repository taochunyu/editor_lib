use std::rc::Rc;
use crate::node::element_type::ElementType;

pub struct Root;

impl ElementType for Root {
    type Attributes = ();

    fn name() -> &'static str {
        "root"
    }
}
