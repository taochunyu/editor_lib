use std::rc::Rc;
use renderer::html::div::HTMLDivElement;
use renderer::Renderer;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

pub struct Root;

#[derive(Eq, PartialEq)]
pub struct RootAttributes;

impl RootAttributes {
    pub fn new() -> Self {
        Self
    }
}

impl ElementType for Root {
    type Attributes = RootAttributes;

    fn name() -> &'static str {
        "root"
    }

    fn render(renderer: Rc<Renderer>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let outer = renderer.create_element::<HTMLDivElement>();

        (outer.clone().into(), Some(outer.into()))
    }
}
