pub mod host;
pub mod html;

use std::rc::Rc;
use crate::host::Host;
use crate::html::{HtmlElement, HtmlTextNode, HtmlVoidElement};
use crate::html::tag::{HtmlElementTag, HtmlVoidElementTag};

pub struct Renderer {
    host: Rc<dyn Host>,
    root: HtmlElement,
}

impl Renderer {
    pub fn new(host: Rc<dyn Host>) -> Self {
        let root = HtmlElement::new(host.clone(), host.root_instance());

        Self { host, root }
    }

    pub fn create_element<T: HtmlElementTag>(&self) -> HtmlElement {
        T::create(self.host.clone())
    }

    pub fn create_void_element<T: HtmlVoidElementTag>(&self) -> HtmlVoidElement {
        T::create(self.host.clone())
    }

    pub fn create_text_node(&self, content: &str) -> HtmlTextNode {
        let instance = self.host.create_text_instance(content);

        HtmlTextNode::new(self.host.clone(), instance)
    }

    pub fn root(&self) -> &HtmlElement {
        &self.root
    }
}
