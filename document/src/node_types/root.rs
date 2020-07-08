use std::rc::Rc;
use renderer::host::Host;
use renderer::html::div::Div;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;
use renderer::html::HtmlNode;

pub struct Root;

impl ElementType for Root {
    type Attributes = ();

    fn name() -> &'static str {
        "root"
    }

    fn render<H: Host>(view: Rc<View<H>>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM<H>, ContentDOM<H>) {
        let outer = view.renderer().create_element::<Div>();

        (Rc::new(HtmlNode::from(outer)), Some(outer.clone()))
    }
}
