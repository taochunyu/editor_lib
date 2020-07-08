use std::rc::Rc;
use renderer::html::p::P;
use renderer::html::HtmlNode;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn render(view: Rc<View>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let paragraph = view.renderer().create_element::<P>();

        (HtmlNode::from(paragraph.clone()), Some(paragraph))
    }
}
