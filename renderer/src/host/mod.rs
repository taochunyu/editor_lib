use std::any::Any;
use std::rc::Rc;

pub trait HostInstance {
    fn as_any(&self) -> &dyn Any;
}

pub trait Host {
    fn root_instance(&self) -> Rc<dyn HostInstance>;

    fn create_instance(&self, name: &str) -> Rc<dyn HostInstance>;

    fn create_text_instance(&self, content: &str) -> Rc<dyn HostInstance>;

    fn append_child(&self, parent: &Rc<dyn HostInstance>, child: &Rc<dyn HostInstance>);
}
