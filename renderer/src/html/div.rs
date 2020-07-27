use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::tag::HTMLElementTag;
use crate::html::node::HTMLNode;
use crate::html::element::HTMLElement;

const NAME: &str = "div";

#[derive(Clone)]
pub struct HTMLDivElement {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

impl From<HTMLDivElement> for HTMLElement {
    fn from(div: HTMLDivElement) -> Self {
        Self {
            tag_name: NAME,
            host: div.host.clone(),
            instance: div.instance.clone(),
        }
    }
}

impl From<HTMLDivElement> for HTMLNode {
    fn from(element: HTMLDivElement) -> Self {
        Self {
            host: element.host.clone(),
            instance: element.instance.clone(),
        }
    }
}

impl HTMLElementTag<HTMLDivElement> for HTMLDivElement {
    fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> HTMLDivElement {
        Self { host, instance }
    }

    fn create(host: Rc<dyn Host>) -> Self {
        let instance = host.create_instance(NAME, &vec![]);

        Self { host, instance }
    }
}

impl HTMLDivElement {
    pub fn append_child(&self, child: &HTMLNode) -> &Self {
        self.host.append_child(self.instance.clone(), child.instance.clone());

        self
    }
}
