use std::rc::Rc;
use renderer::html::div::HTMLDivElement;
use renderer::Renderer;
use crate::schema::node_type::NodeType;
use crate::node::Node;
use crate::view::View;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;

pub struct Root;

#[derive(Eq, PartialEq)]
pub struct RootAttributes;

impl RootAttributes {
    pub fn new() -> Self {
        Self
    }
}

impl NodeType for Root {
    type Attributes = RootAttributes;
    type DOM = HTMLDivElement;

    fn name() -> &'static str {
        "root"
    }

    fn render(renderer: Rc<Renderer>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (HTMLNode, Option<HTMLElement>) {
        let outer = renderer.create_element::<HTMLDivElement>();

        (outer.clone().into(), Some(outer.into()))
    }
}
