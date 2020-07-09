use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::node::HTMLNode;

#[derive(Clone)]
pub struct HTMLTextNode {
    pub(crate) host: Rc<dyn Host>,
    pub(crate) instance: Rc<dyn HostInstance>,
}

impl From<HTMLTextNode> for HTMLNode {
    fn from(text_node: HTMLTextNode) -> Self {
        Self {
            host: text_node.host.clone(),
            instance: text_node.instance.clone(),
        }
    }
}

impl HTMLTextNode {
    pub(crate) fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> Self {
        Self { host, instance, }
    }
}
