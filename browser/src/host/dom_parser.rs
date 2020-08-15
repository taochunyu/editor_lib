use renderer::html::any_node::{HTMLAnyNode, HTMLNodeType};
use wasm_bindgen::JsCast;

struct AnyNode {
    node: web_sys::Node,
}

impl AnyNode {
    fn to_element(&self) -> Option<web_sys::Element> {
        match self.node.clone().dyn_into::<web_sys::Element>() {
            Ok(element) => Some(element),
            _ => None,
        }
    }
}

impl HTMLAnyNode for AnyNode {
    fn node_type(&self) -> HTMLNodeType {
        match self.node.node_type() {
            1 => HTMLNodeType::Element,
            3 => HTMLNodeType::Text,
            _ => HTMLNodeType::Other,
        }
    }

    fn tag_name(&self) -> Option<String> {
        self.to_element().map(|elm| elm.tag_name())
    }

    fn text_content(&self) -> Option<String> {
        self.node.text_content()
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        self.to_element().map_or(None, |elm| elm.get_attribute(name))
    }
}

pub struct DOMParser {

}

