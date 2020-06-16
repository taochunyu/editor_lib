use crate::node::node_type::NodeType;
use std::rc::Rc;
use crate::node::Node;

struct TextNodeType;

impl NodeType for TextNodeType {
    type Attributes = String;

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

struct ParagraphNodeType;

impl NodeType for ParagraphNodeType {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

#[test]
fn it_works() {

}