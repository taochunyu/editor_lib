use crate::html::HtmlElement;
use crate::host::Host;
use crate::html::tag::HtmlElementTag;

pub struct P;

impl<H: Host> HtmlElementTag<H> for P {
    fn create(host: &H) -> HtmlElement<H> {
        let instance = host.create_instance("p");

        HtmlElement::new(instance)
    }
}
