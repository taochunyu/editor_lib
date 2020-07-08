use crate::host::Host;
use crate::html::{HtmlElement, HtmlVoidElement};

pub trait HtmlElementTag<H: Host> {
    fn create(host: &H) -> HtmlElement<H>;
}

pub trait HtmlVoidElementTag<H: Host> {
    fn create(host: &H) -> HtmlVoidElement<H>;
}
