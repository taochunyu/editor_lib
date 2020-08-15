use std::rc::Rc;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use renderer::Renderer;
use renderer::html::any_node::HTMLAnyNode;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use crate::view::View;

// pub struct NodeSchema {
//     name: String,
//     parse_html: Box<dyn Fn() -> Slice>
// }

pub trait NodeType: Sized + 'static {
    type Attributes: PartialEq + Eq;
    type DOM: Into<HTMLNode>;
    // type ContentDOM: Into<HTMLElement>;

    fn name() -> &'static str;

    fn parse_from_html(node: Box<dyn HTMLAnyNode>) -> Option<Rc<Self::Attributes>>;

    fn serialize_to_html(attrs: Rc<Self::Attributes>, children: String) -> String {
        String::new()
    }

    fn create(
        attrs: Rc<Self::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>
    ) -> Rc<dyn Node> {
        let element_children = match children {
            Some(nodes) => Some(Rc::new(Fragment::from(nodes))),
            None => None,
        };

        Element::<Self>::new(attrs, element_children)
    }

    fn render(renderer: Rc<Renderer>, node: Rc<dyn Node>, attrs: Rc<Self::Attributes>) -> (HTMLNode, Option<HTMLElement>);
}
