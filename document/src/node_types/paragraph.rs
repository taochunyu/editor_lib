use std::rc::Rc;
use renderer::host::Host;
use renderer::html::p::P;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;
use renderer::html::HtmlNode;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn render<H: Host>(view: Rc<View<H>>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM<H>, ContentDOM<H>) {
        let paragraph = view.renderer().create_element::<P>();

        (Rc::new(HtmlNode::from(paragraph)), Some(paragraph.clone()))
    }
}
