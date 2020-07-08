use std::rc::Rc;
use crate::host::Host;
use crate::html::HtmlVoidElement;
use crate::html::tag::HtmlVoidElementTag;

pub struct Img;

impl HtmlVoidElementTag for Img {
    fn create(host: Rc<dyn Host>) -> HtmlVoidElement {
        let instance = host.create_instance("div");

        HtmlVoidElement::new(host, instance)
    }
}
