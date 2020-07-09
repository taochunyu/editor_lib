use std::rc::Rc;
use crate::host::{Host, HostInstance};

#[derive(Clone)]
pub struct HTMLNode {
    pub(crate) host: Rc<dyn Host>,
    pub(crate) instance: Rc<dyn HostInstance>,
}

impl HTMLNode {
    pub fn host(&self) -> Rc<dyn Host> {
        self.host.clone()
    }

    pub fn instance(&self) -> &Rc<dyn HostInstance> {
        &self.instance
    }
}