pub mod img;
pub mod tag;
pub mod p;
pub mod div;

use std::rc::Rc;
use crate::host::{Host, HostInstance};

#[derive(Clone)]
pub struct HtmlNode {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

#[derive(Clone)]
pub struct HtmlTextNode {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

#[derive(Clone)]
pub struct HtmlElement {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

#[derive(Clone)]
pub struct HtmlVoidElement {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

impl From<HtmlTextNode> for HtmlNode {
    fn from(text_node: HtmlTextNode) -> Self {
        Self {
            host: text_node.host.clone(),
            instance: text_node.instance.clone(),
        }
    }
}

impl From<HtmlElement> for HtmlNode {
    fn from(html_element: HtmlElement) -> Self {
        Self {
            host: html_element.host.clone(),
            instance: html_element.instance.clone(),
        }
    }
}

impl From<HtmlVoidElement> for HtmlNode {
    fn from(html_void_element: HtmlVoidElement) -> Self {
        Self {
            host: html_void_element.host.clone(),
            instance: html_void_element.instance.clone(),
        }
    }
}

impl HtmlNode {
    pub fn host(&self) -> Rc<dyn Host> {
        self.host.clone()
    }
    pub fn instance(&self) -> &Rc<dyn HostInstance> {
        &self.instance
    }
}

impl HtmlTextNode {
    pub(crate) fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> Self {
        Self { host, instance, }
    }
}

impl HtmlElement {
    pub(crate) fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> Self {
        Self { host, instance }
    }

    pub fn append_child(&self, child: &HtmlNode) -> &Self {
        self.host.append_child(&self.instance, &child.instance);

        self
    }
}

impl HtmlVoidElement {
    pub(crate) fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> Self {
        Self {
            host,
            instance,
        }
    }
}
