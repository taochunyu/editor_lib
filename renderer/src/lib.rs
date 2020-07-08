pub mod host;
pub mod html;

use crate::host::Host;
use crate::html::{HtmlElement, HtmlTextNode, HtmlVoidElement};
use crate::html::tag::{HtmlElementTag, HtmlVoidElementTag};

pub struct Renderer<H: Host> {
    host: H,
    root: HtmlElement<H>,
}

impl<H: Host> Renderer<H> {
    pub fn new(host: H) -> Self {
        let root = HtmlElement::new(host.root_instance());

        Self { host, root }
    }

    pub fn create_element<T: HtmlElementTag<H>>(&self) -> HtmlElement<H> {
        T::create(&self.host)
    }

    pub fn create_void_element<T: HtmlVoidElementTag<H>>(&self) -> HtmlVoidElement<H> {
        T::create(&self.host)
    }

    pub fn create_text_node(&self, content: &str) -> HtmlTextNode<H> {
        let instance = self.host.create_text_instance(content);

        HtmlTextNode::new(instance)
    }

    pub fn root(&self) -> &HtmlElement<H> {
        &self.root
    }
}
