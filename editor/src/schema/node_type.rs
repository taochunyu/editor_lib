use std::rc::Rc;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use renderer::Renderer;
use renderer::html::any_node::HTMLAnyNode;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use crate::view::View;

pub type Match = Box<dyn Fn(&Box<dyn HTMLAnyNode>) -> bool>;
pub type Parse<Result> = Box<dyn Fn(&Box<dyn HTMLAnyNode>) -> Rc<Result>>;
pub type ParseRules<Result> = Vec<(Match, Parse<Result>)>;

pub trait NodeType: Sized + 'static {
    type Attributes: PartialEq + Eq;
    type DOM: Into<HTMLNode>;
    // type ContentDOM: Into<HTMLElement>;

    fn name() -> &'static str;

    fn parse_from_html() -> ParseRules<Self::Attributes> { unimplemented!() }

    fn serialize_to_html(attrs: Rc<Self::Attributes>, children: String) -> String { unimplemented!() }

    fn render(renderer: Rc<Renderer>, node: Rc<dyn Node>, attrs: Rc<Self::Attributes>) -> (HTMLNode, Option<HTMLElement>);
}
