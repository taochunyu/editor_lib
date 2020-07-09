use web_sys::window;
use renderer::host::{Host, HostInstance, Attributes};
use std::rc::Rc;
use std::any::Any;

struct BrowserHostInstance {
    host_instance: web_sys::Node,
}

pub struct Browser {
    document: web_sys::Document,
    root_id: &'static str,
}

impl HostInstance for BrowserHostInstance {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Host for Browser {
    fn root_instance(&self) -> Rc<dyn HostInstance> {
        let document = window().unwrap().document().unwrap();
        let host_instance: web_sys::Node = document.query_selector(self.root_id).unwrap().unwrap().into();

        Rc::new(BrowserHostInstance { host_instance })
    }

    fn create_instance(&self, name: &str, attrs: &Attributes) -> Rc<dyn HostInstance> {
        let host_instance = self.document.create_element(name).unwrap();

        for (key, value) in attrs {
            host_instance.set_attribute(key.as_str(), value.as_str());
        }

        Rc::new(BrowserHostInstance { host_instance: web_sys::Node::from(host_instance) })
    }

    fn create_text_instance(&self, content: &str) -> Rc<dyn HostInstance> {
        let host_instance: web_sys::Node = self.document.create_text_node(content).into();

        Rc::new(BrowserHostInstance { host_instance })
    }

    fn append_child(&self, parent: &Rc<dyn HostInstance>, child: &Rc<dyn HostInstance>) {
        let parent: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let child: &BrowserHostInstance = child.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        parent.host_instance.append_child(&child.host_instance);
    }
}

impl Browser {
    pub fn new(root_id: &'static str) -> Rc<dyn Host> {
        let document = window().unwrap().document().unwrap();

        Rc::new(Self { document, root_id })
    }
}
