use std::rc::Rc;
use renderer::html::p::HTMLParagraphElement;
use renderer::Renderer;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

const NAME: &'static str = "paragraph";

#[derive(Eq, PartialEq)]
pub struct ParagraphAttributes;

impl ParagraphAttributes {
    pub fn new() -> Self {
        Self
    }
}

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ParagraphAttributes;

    fn name() -> &'static str {
        NAME
    }

    fn render(renderer: Rc<Renderer>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let paragraph = renderer.create_element::<HTMLParagraphElement>();

        (paragraph.clone().into(), Some(paragraph.into()))
    }
}
