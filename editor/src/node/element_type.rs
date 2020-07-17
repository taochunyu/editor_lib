use std::rc::Rc;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use crate::view::View;

pub type OuterDOM = HTMLNode;
pub type ContentDOM = Option<HTMLElement>;

pub trait ElementType: Sized + 'static {
    type Attributes;

    fn name() -> &'static str;

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

    fn render(view: Rc<View>, node: Rc<dyn Node>, attrs: Rc<Self::Attributes>) -> (OuterDOM, ContentDOM);
}
