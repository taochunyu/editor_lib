use std::rc::Rc;
use renderer::html::HtmlNode;
use renderer::html::div::HtmlDivElement;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::view::View;

pub struct Root;

impl ElementType for Root {
    type Attributes = ();

    fn name() -> &'static str {
        "root"
    }

    fn render(view: Rc<View>, _node: Rc<dyn Node>, _attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM) {
        let outer = view.renderer().create_element::<HtmlDivElement>();

        (HtmlNode::from(outer.clone()), Some(outer))
    }
}
