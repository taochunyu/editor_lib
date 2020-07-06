use std::rc::Rc;
use std::cell::RefCell;
use crate::html::{HtmlNode, HtmlNodeType};
use crate::host::Host;

pub struct Div<H: Host> {
    render_host: H,
    instance:
}

impl<H: Host> HtmlNodeType for Div<H> {
    type Attributes = ();

    fn name() -> &'static str {
        "div"
    }

    fn new(host: H,_attrs: Self::Attributes) -> Self {
        Self
    }

    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        self.children.clone()
    }

    fn append_child(&mut self, child: Rc<RefCell<dyn Node>>) {
        self.children.push(child);
    }
}