use crate::node::element_type::ElementType;
use std::rc::Rc;

struct Root;

impl ElementType for Root {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Root)
    }
}
