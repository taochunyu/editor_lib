use std::rc::Rc;
use std::any::Any;
use crate::host::{HostInstance, Host, Attributes};

struct DebugHostInstance;

pub struct DebugHost;

impl HostInstance for DebugHostInstance {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq(&self, other: Rc<dyn HostInstance>) -> bool {
        false
    }
}

impl Host for DebugHost {
    fn log(&self, info: String) {
        println!("{}", info)
    }

    fn root_instance(&self) -> Rc<dyn HostInstance> {
        Rc::new(DebugHostInstance)
    }

    fn create_instance(&self, _name: &str, _attrs: &Attributes) -> Rc<dyn HostInstance> {
        Rc::new(DebugHostInstance)
    }

    fn create_text_instance(&self, _content: &str) -> Rc<dyn HostInstance> {
        Rc::new(DebugHostInstance)
    }

    fn append_child(&self, _parent: Rc<dyn HostInstance>, _child: Rc<dyn HostInstance>) {}

    fn insert_before(&self, parent: Rc<dyn HostInstance>, instance: Rc<dyn HostInstance>, child: Option<Rc<dyn HostInstance>>) {}

    fn remove_child(&self, parent: Rc<dyn HostInstance>, child: Rc<dyn HostInstance>) {}

    fn next_sibling(&self, _instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        None
    }

    fn first_child(&self, _parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        None
    }

    fn last_child(&self, _parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        None
    }

    fn parent(&self, _instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        None
    }

    fn node_value(&self, instance: Rc<dyn HostInstance>) -> Option<String> {
        None
    }

    fn set_attribute(&self, instance: Rc<dyn HostInstance>, name: &str, value: &str) {}

    fn set_node_value(&self, instance: Rc<dyn HostInstance>, value: Option<&str>) {}

    fn set_selection(&self, anchor_instance: Rc<dyn HostInstance>, anchor_offset: usize, head_instance: Rc<dyn HostInstance>, head_offset: usize) {}
}

impl DebugHost {
    pub fn new() -> Rc<dyn Host> {
        Rc::new(DebugHost)
    }
}
