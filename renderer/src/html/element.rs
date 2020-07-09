use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::node::HTMLNode;

#[derive(Clone)]
pub struct HTMLElement {
    pub(crate) host: Rc<dyn Host>,
    pub(crate) instance: Rc<dyn HostInstance>,
}

impl From<HTMLElement> for HTMLNode {
    fn from(html_element: HTMLElement) -> Self {
        Self {
            host: html_element.host.clone(),
            instance: html_element.instance.clone(),
        }
    }
}

impl HTMLElement {
    pub fn host(&self) -> Rc<dyn Host> {
        self.host.clone()
    }

    pub fn instance(&self) -> &Rc<dyn HostInstance> {
        &self.instance
    }

    pub fn append_child(&self, child: &HTMLNode) -> &Self {
        self.host.append_child(&self.instance, &child.instance);

        self
    }
}