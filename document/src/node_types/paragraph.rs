use std::rc::Rc;
use crate::node::element_type::ElementType;
use crate::node::Node;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();
    type State = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn render(_: Self::State, node: Rc<dyn Node>) -> (OuterDOM, ContentDOM) {

    }
}
