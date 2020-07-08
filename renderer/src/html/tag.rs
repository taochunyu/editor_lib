use std::rc::Rc;
use crate::host::Host;
use crate::html::{HtmlElement, HtmlVoidElement};

pub trait HtmlElementTag {
    fn create(host: Rc<dyn Host>) -> HtmlElement;
}

pub trait HtmlVoidElementTag {
    fn create(host: Rc<dyn Host>) -> HtmlVoidElement;
}
