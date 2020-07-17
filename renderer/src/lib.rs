pub mod host;
pub mod html;

use std::rc::Rc;
use crate::host::Host;
use crate::html::tag::HTMLElementTag;
use crate::html::div::HTMLDivElement;
use crate::html::text::HTMLTextNode;

pub struct Renderer {
    host: Rc<dyn Host>,
    root: HTMLDivElement,
}

impl Renderer {
    pub fn new(host: Rc<dyn Host>) -> Self {
        let root = HTMLDivElement::new(host.clone(), host.root_instance());

        Self { host, root }
    }

    pub fn create_element<T: HTMLElementTag<T>>(&self) -> T {
        T::create(self.host.clone())
    }

    pub fn create_text_node(&self, content: &str) -> HTMLTextNode {
        let instance = self.host.create_text_instance(content);

        HTMLTextNode::new(self.host.clone(), instance)
    }

    pub fn root(&self) -> &HTMLDivElement {
        &self.root
    }
}
