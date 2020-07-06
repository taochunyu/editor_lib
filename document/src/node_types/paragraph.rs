use std::rc::Rc;
use renderer::html::paragraph;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn render(view: Rc<View>, node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let paragraph = view.ui().create_element::<paragraph::Paragraph>(());

        (paragraph.clone(), Some(paragraph.clone()))
    }
}
