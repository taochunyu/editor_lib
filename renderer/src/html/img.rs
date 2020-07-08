use crate::html::HtmlVoidElement;
use crate::host::Host;
use crate::html::tag::HtmlVoidElementTag;

pub struct Div;

impl<H: Host> HtmlVoidElementTag<H> for Div {
    fn create(host: &H) -> HtmlVoidElement<H> {
        let instance = host.create_instance("div");

        HtmlVoidElement::new(instance)
    }
}
