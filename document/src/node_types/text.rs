use std::rc::Rc;
use crate::node;
use crate::node::Node;
use crate::node::text_type::TextType;

struct Text;

impl TextType for Text {
    fn new() -> Rc<Self> {
        Rc::new(Text)
    }
}
