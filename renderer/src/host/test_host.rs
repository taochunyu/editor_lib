use std::rc::Rc;
use std::any::Any;
use crate::host::{Host, HostInstance, Attributes};


struct TestHostInstance;

pub struct TestHost;

impl HostInstance for TestHostInstance {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Host for TestHost {
    fn root_instance(&self) -> Rc<dyn HostInstance> {
        Rc::new(TestHostInstance)
    }

    fn create_instance(&self, _name: &str, _attrs: &Attributes) -> Rc<dyn HostInstance> {
        Rc::new(TestHostInstance)
    }

    fn create_text_instance(&self, _content: &str) -> Rc<dyn HostInstance> {
        Rc::new(TestHostInstance)
    }

    fn append_child(&self, _parent: &Rc<dyn HostInstance>, _child: &Rc<dyn HostInstance>) {}
}

impl TestHost {
    pub fn new() -> Rc<dyn Host> {
        Rc::new(TestHost)
    }
}
