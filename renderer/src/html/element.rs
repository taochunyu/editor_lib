use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::node::HTMLNode;

#[derive(Clone)]
pub struct HTMLElement {
    pub(crate) tag_name: &'static str,
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
    pub fn log(&self, info: String) {
        self.host.log(info);
    }

    pub fn first_child(&self) -> Option<HTMLNode> {
        self.host.first_child(self.instance.clone()).map(|instance| HTMLNode {
            host: self.host.clone(),
            instance,
        })
    }

    pub fn last_child(&self) -> Option<HTMLNode> {
        self.host.last_child(self.instance.clone()).map(|instance| HTMLNode {
            host: self.host.clone(),
            instance,
        })
    }

    pub fn append_child(&self, child: HTMLNode) -> &Self {
        self.host.append_child(self.instance.clone(), child.instance.clone());

        self
    }

    pub fn insert_before(&self, node: HTMLNode, child: Option<HTMLNode>) -> &Self {
        let child = child.map(|node| node.instance.clone());

        self.host.insert_before(self.instance.clone(), node.instance.clone(), child);

        self
    }
}
