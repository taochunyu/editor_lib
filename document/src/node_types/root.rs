use std::rc::Rc;
use renderer::html::div::Div;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

pub struct Root;

impl ElementType for Root {
    type Attributes = ();

    fn name() -> &'static str {
        "root"
    }

    fn render(view: Rc<View>, node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let outer = view.ui().create_element::<Div>(());

        (outer.clone(), Some(outer.clone()))
    }
}
