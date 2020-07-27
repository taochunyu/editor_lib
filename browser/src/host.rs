use web_sys;
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

    fn eq(&self, other: Rc<dyn HostInstance>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self.host_instance == other.host_instance
        } else {
            false
        }
    }
}

impl Host for Browser {
    fn log(&self, info: String) {
        web_sys::console::log_1(&info.into())
    }

    fn root_instance(&self) -> Rc<dyn HostInstance> {
        let document = web_sys::window().unwrap().document().unwrap();
        let host_instance: web_sys::Node = document.query_selector(self.root_id).unwrap().unwrap().into();

        Rc::new(BrowserHostInstance { host_instance })
    }

    fn create_instance(&self, name: &str, attrs: &Attributes) -> Rc<dyn HostInstance> {
        let host_instance = self.document.create_element(name).unwrap();

        for (key, value) in attrs {
            host_instance.set_attribute(key.as_str(), value.as_str()).unwrap();
        }

        Rc::new(BrowserHostInstance { host_instance: web_sys::Node::from(host_instance) })
    }

    fn create_text_instance(&self, content: &str) -> Rc<dyn HostInstance> {
        let host_instance: web_sys::Node = self.document.create_text_node(content).into();

        Rc::new(BrowserHostInstance { host_instance })
    }

    fn append_child(&self, parent: Rc<dyn HostInstance>, child: Rc<dyn HostInstance>) {
        let parent: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let child: &BrowserHostInstance = child.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        parent.host_instance.append_child(&child.host_instance).unwrap();
    }

    fn insert_before(&self, parent: Rc<dyn HostInstance>, instance: Rc<dyn HostInstance>, child: Option<Rc<dyn HostInstance>>) {
        let parent: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let instance: &BrowserHostInstance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let child: Option<web_sys::Node> = match child {
            Some(instance) => {
                let instance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

                Some(instance.host_instance.clone())
            },
            None => None,
        };

        parent.host_instance.insert_before(&instance.host_instance, child.as_ref());
    }

    fn remove_child(&self, parent: Rc<dyn HostInstance>, child: Rc<dyn HostInstance>) {
        let parent: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let child: &BrowserHostInstance = child.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        parent.host_instance.remove_child(&child.host_instance);
    }

    fn next_sibling(&self, instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        let node: &BrowserHostInstance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let next_sibling: Option<web_sys::Node> = node.host_instance.next_sibling();

        next_sibling.map(|host_instance| Rc::new(BrowserHostInstance { host_instance }) as Rc<dyn HostInstance>)
    }

    fn first_child(&self, parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        let node: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let first_child: Option<web_sys::Node> = node.host_instance.first_child();

        first_child.map(|host_instance| Rc::new(BrowserHostInstance { host_instance }) as Rc<dyn HostInstance>)
    }

    fn last_child(&self, parent: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        let node: &BrowserHostInstance = parent.as_any().downcast_ref::<BrowserHostInstance>().unwrap();
        let last_child: Option<web_sys::Node> = node.host_instance.last_child();

        last_child.map(|host_instance| Rc::new(BrowserHostInstance { host_instance }) as Rc<dyn HostInstance>)
    }

    fn parent(&self, instance: Rc<dyn HostInstance>) -> Option<Rc<dyn HostInstance>> {
        let node: &BrowserHostInstance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        node.host_instance.parent_node().map(|host_instance| Rc::new(BrowserHostInstance { host_instance }) as Rc<dyn HostInstance>)
    }

    fn node_value(&self, instance: Rc<dyn HostInstance>) -> Option<String> {
        let node: &BrowserHostInstance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        node.host_instance.node_value()
    }

    fn set_node_value(&self, instance: Rc<dyn HostInstance>, value: Option<&str>) {
        let node: &BrowserHostInstance = instance.as_any().downcast_ref::<BrowserHostInstance>().unwrap();

        node.host_instance.set_node_value(value);
    }
}

impl Browser {
    pub fn new(root_id: &'static str) -> Rc<dyn Host> {
        let document = web_sys::window().unwrap().document().unwrap();

        Rc::new(Self { document, root_id })
    }
}
