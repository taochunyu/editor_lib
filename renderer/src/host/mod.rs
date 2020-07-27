use std::any::Any;
use std::rc::Rc;

pub trait HostInstance {
    fn as_any(&self) -> &dyn Any;
    fn eq(&self, other: Rc<dyn HostInstance>) -> bool;
}

pub type AttributeKey = String;
pub type AttributeValue = String;
pub type Attributes = Vec<(AttributeKey, AttributeValue)>;

pub trait Host {
    fn log(&self, info: String);

    fn root_instance(&self) -> Rc<dyn HostInstance>;

    fn create_instance(&self, name: &str, attrs: &Attributes) -> Rc<dyn HostInstance>;

    fn create_text_instance(&self, content: &str) -> Rc<dyn HostInstance>;

    fn append_child(&self, parent: Rc<dyn HostInstance>, child: Rc<dyn HostInstance>);

    fn insert_before(&self, parent: Rc<dyn HostInstance>, instance: Rc<dyn HostInstance>, child: Option<Rc<dyn HostInstance>>);

    fn remove_child(&self, parent: Rc<dyn HostInstance>, child: Rc<dyn HostInstance>);

    fn next_sibling(&self, instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>>;

    fn first_child(&self, parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>>;

    fn last_child(&self, parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>>;

    fn parent(&self, instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>>;

    fn node_value(&self, instance: Rc<dyn HostInstance>) -> Option<String>;

    fn set_node_value(&self, instance: Rc<dyn HostInstance>, value: Option<&str>);
}
