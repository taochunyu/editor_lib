use std::rc::Rc;
use crate::host::Host;
use crate::html::HtmlElement;
use crate::html::tag::HtmlElementTag;

pub struct P;

impl HtmlElementTag for P {
    fn create(host: Rc<dyn Host>) -> HtmlElement {
        let instance = host.create_instance("p");

        HtmlElement::new(host, instance)
    }
}
