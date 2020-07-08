pub mod img;
pub mod tag;
pub mod p;
pub mod div;

use crate::host::Host;

pub struct HtmlNode<H: Host> {
    instance: H::Instance,
}

pub struct HtmlTextNode<H: Host> {
    instance: H::Instance,
}

pub struct HtmlElement<H: Host> {
    instance: H::Instance,
}

pub struct HtmlVoidElement<H: Host> {
    instance: H::Instance,
}

impl<H: Host> From<HtmlTextNode<H>> for HtmlNode<H> {
    fn from(text_node: HtmlTextNode<H>) -> Self {
        Self { instance: text_node.instance }
    }
}

impl<H: Host> From<HtmlElement<H>> for HtmlNode<H> {
    fn from(html_element: HtmlElement<H>) -> Self {
        Self { instance: html_element.instance }
    }
}

impl<H: Host> From<HtmlVoidElement<H>> for HtmlNode<H> {
    fn from(html_void_element: HtmlVoidElement<H>) -> Self {
        Self { instance: html_void_element.instance }
    }
}

impl<H: Host> HtmlNode<H> {
    pub fn instance(&self) -> &H::Instance {
        &self.instance
    }
}

impl<H: Host> HtmlTextNode<H> {
    pub(crate) fn new(instance: H::Instance) -> Self {
        Self { instance }
    }
}

impl<H: Host> HtmlElement<H> {
    pub(crate) fn new(instance: H::Instance) -> Self {
        Self { instance }
    }

    pub fn append_child(&self, child: &HtmlNode<H>) -> &Self {
        H::append_child(&self.instance, &child.instance);

        self
    }
}

impl<H: Host> HtmlVoidElement<H> {
    pub(crate) fn new(instance: H::Instance) -> Self {
        Self { instance }
    }
}
