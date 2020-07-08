use std::rc::Rc;
use crate::host::Host;
use crate::html::HtmlElement;
use crate::html::tag::HtmlElementTag;

pub struct Div;

impl HtmlElementTag for Div {
    fn create(host: Rc<dyn Host>) -> HtmlElement {
        let instance = host.create_instance("div");

        HtmlElement::new(host, instance)
    }
}
