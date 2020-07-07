pub mod host;
pub mod html;

use crate::host::Host;
use crate::html::{Tag, HtmlNode};

pub struct Renderer<H: Host> {
    host: H,
    root: HtmlNode<H>,
}

impl<H: Host> Renderer<H> {
    pub fn new(host: H) -> Self {
        let root = HtmlNode::new(host.root_instance());

        Self { host, root }
    }

    pub fn create_element<T: Tag<H>>(&self) -> HtmlNode<H> {
        T::create(&self.host)
    }

    pub fn create_text_node(&self, content: &str) -> HtmlNode<H> {
        let instance = self.host.create_text_instance(content);

        HtmlNode::new(instance)
    }

    pub fn root(&self) -> &HtmlNode<H> {
        &self.root
    }
}
