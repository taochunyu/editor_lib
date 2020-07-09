use std::rc::Rc;
use crate::host::{Host, HostInstance};
use crate::html::tag::HTMLElementTag;
use crate::html::element::HTMLElement;
use crate::html::node::HTMLNode;

pub struct HTMLParagraphElement {
    host: Rc<dyn Host>,
    instance: Rc<dyn HostInstance>,
}

impl From<HTMLParagraphElement> for HTMLElement {
    fn from(div: HTMLParagraphElement) -> Self {
        Self {
            host: div.host.clone(),
            instance: div.instance.clone(),
        }
    }
}

impl From<HTMLParagraphElement> for HTMLNode {
    fn from(element: HTMLParagraphElement) -> Self {
        Self {
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
        let instance = host.create_instance("div", &vec![]);

        Self { host, instance }
    }
}
