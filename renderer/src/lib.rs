pub mod host;
pub mod html;

use std::rc::Rc;
use std::cell::RefCell;
use crate::host::Host;
use crate::html::HtmlNodeType;

pub struct Renderer<H: Host> {
    host: H,
    root: H::Instance,
}

impl<H: Host> Renderer<H> {
    pub fn new(host: H) -> Self {
        let root = H::create_root_instance();

        Self { host, root }
    }

    pub fn create_element<T: HtmlNodeType>(
        &mut self,
        attrs: <T as HtmlNodeType>::Attributes
    ) -> {
    }

    pub fn root(&self) -> Rc<RefCell<dyn Node>> {
        self.root_element.clone()
    }
}
