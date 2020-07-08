use crate::html::HtmlElement;
use crate::host::Host;
use crate::html::tag::HtmlElementTag;

pub struct Div;

impl<H: Host> HtmlElementTag<H> for Div {
    fn create(host: &H) -> HtmlElement<H> {
        let instance = host.create_instance("div");

        HtmlElement::new(instance)
    }
}
