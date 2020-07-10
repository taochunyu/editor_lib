use std::rc::Rc;
use crate::host::{Host, HostInstance};

#[derive(Clone)]
pub struct HTMLNode {
    pub(crate) name: &'static str,
    pub(crate) host: Rc<dyn Host>,
    pub(crate) instance: Rc<dyn HostInstance>,
}

impl HTMLNode {}
