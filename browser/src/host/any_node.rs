use renderer::html::any_node::{HTMLAnyNode, HTMLNodeType};
use wasm_bindgen::JsCast;
use editor::editor::Editor;
use editor::node::Node;
use std::rc::Rc;
use renderer::html::node::HTMLNode;
use crate::host::host::BrowserHostInstance;

pub struct AnyNode {
    node: web_sys::Node,
}

impl From<HTMLNode> for AnyNode {
    fn from(node: HTMLNode) -> Self {
        match node.instance().as_any().downcast_ref::<BrowserHostInstance>() {
            Some(instance) => AnyNode { node: instance.node().clone() },
            None => unreachable!("Downcast to web_sys node failed."),
        }
    }
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

    fn children(&self) -> Option<Vec<Box<dyn HTMLAnyNode>>> {
        unimplemented!()
    }
}
