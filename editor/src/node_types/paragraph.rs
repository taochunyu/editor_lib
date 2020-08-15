use std::rc::Rc;
use renderer::html::p::HTMLParagraphElement;
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::schema::node_type::NodeType;
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

impl NodeType for Paragraph {
    type Attributes = ParagraphAttributes;
    type DOM = HTMLParagraphElement;

    fn name() -> &'static str {
        NAME
    }

    fn render(renderer: Rc<Renderer>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (HTMLNode, Option<HTMLElement>) {
        let paragraph = renderer.create_element::<HTMLParagraphElement>();

        (paragraph.clone().into(), Some(paragraph.into()))
    }
}
