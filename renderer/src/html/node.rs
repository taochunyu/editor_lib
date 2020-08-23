use std::rc::Rc;
use crate::host::{Host, HostInstance, ExtraInfo};

#[derive(Clone)]
pub struct HTMLNode {
    pub(crate) host: Rc<dyn Host>,
    pub(crate) instance: Rc<dyn HostInstance>,
}

impl PartialEq for HTMLNode {
    fn eq(&self, other: &Self) -> bool {
        self.instance.as_ref().eq(other.instance.clone())
    }
}

impl Eq for HTMLNode {}

impl HTMLNode {
    pub fn host(&self) -> Rc<dyn Host> {
        self.host.clone()
    }

    pub fn instance(&self) -> Rc<dyn HostInstance> {
        self.instance.clone()
    }

    pub fn log(&self, info: String) {
       self.host.log(info);
    }

    pub fn next_sibling(&self) -> Option<HTMLNode> {
        self.host.next_sibling(self.instance.clone()).map(|instance| HTMLNode {
            host: self.host.clone(),
            instance,
        })
    }

    pub fn parent(&self) -> Option<HTMLNode> {
        self.host.parent(self.instance.clone()).map(|instance| HTMLNode {
            host: self.host.clone(),
            instance,
        })
    }

    pub fn remove(&self) {
        if let Some(parent) = self.parent() {
            self.host.remove_child(parent.instance.clone(), self.instance.clone());
        }
    }

    pub fn set_extra_info(&self, extra_info: Box<dyn ExtraInfo>) {
        self.host.set_extra_info(self.instance.clone(), extra_info);
    }
}
