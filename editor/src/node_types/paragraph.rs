use std::rc::Rc;
use renderer::html::p::HTMLParagraphElement;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

const NAME: &'static str = "paragraph";

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        NAME
    }

    fn render(view: Rc<View>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let paragraph = view.renderer().create_element::<HTMLParagraphElement>();

        (paragraph.clone().into(), Some(paragraph.into()))
    }
}
