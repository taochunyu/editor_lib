use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::tag::HTMLElementTag;
use crate::html::element::HTMLElement;
use crate::html::node::HTMLNode;

const NAME: &str = "p";

#[derive(Clone)]
pub struct HTMLParagraphElement {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

impl From<HTMLParagraphElement> for HTMLElement {
    fn from(div: HTMLParagraphElement) -> Self {
        Self {
            name: NAME,
            host: div.host.clone(),
            instance: div.instance.clone(),
        }
    }
}

impl From<HTMLParagraphElement> for HTMLNode {
    fn from(element: HTMLParagraphElement) -> Self {
        Self {
            name: NAME,
            host: element.host.clone(),
            instance: element.instance.clone(),
        }
    }
}

impl HTMLElementTag<HTMLParagraphElement> for HTMLParagraphElement {
    fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> HTMLParagraphElement {
        Self { host, instance }
    }

    fn create(host: Rc<dyn Host>) -> Self {
        let instance = host.create_instance(NAME, &vec![]);

        Self { host, instance }
    }
}

impl HTMLParagraphElement {
    pub fn append_child(&self, child: &HTMLNode) -> &Self {
        self.host.append_child(&self.instance, &child.instance);

        self
    }
}
