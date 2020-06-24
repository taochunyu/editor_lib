use std::rc::Rc;
use crate::node::element_type::ElementType;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        "paragraph"
    }
}
