pub mod p;
pub mod div;

use crate::host::Host;

pub trait Tag<H: Host> {
    fn create(host: &H) -> HtmlNode<H>;
}

pub struct HtmlNode<H: Host> {
    instance: H::Instance,
}

impl<H: Host> HtmlNode<H> {
    pub(crate) fn new(instance: H::Instance) -> Self {
        Self { instance }
    }

    pub fn append_child(&self, child: HtmlNode<H>) -> &Self {
        H::append_child(&self.instance, child.instance);

        self
    }
}
