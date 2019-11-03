use std::rc::Rc;
use editor_core::view::virtual_node::VirtualNode;
use crate::dom::element::{Element, ElementType};

pub fn render_virtual_node(virtual_node: &Rc<VirtualNode>) -> Element {
    let element_type = match virtual_node.tag.as_str() {
        "div" => ElementType::Div,
        "p" => ElementType::P,
        "span" => ElementType::Span,
        _ => ElementType::Unknown,
    };

    let mut element = Element::create_element(element_type).unwrap();

    for child in &virtual_node.children {
        match child.tag.as_str() {
            "text" => {
                element.set_inner_html(String::from(&child.props));
                break;
            },
            _ => {
                element.append_child(&mut render_virtual_node(child));
            },
        };
    }

    element
}
