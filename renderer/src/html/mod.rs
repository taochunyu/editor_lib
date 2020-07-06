pub mod text;
pub mod div;
pub mod paragraph;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::instruction::Instruction;
use crate::host::Host;

pub trait HtmlNode {
    fn children(&self) -> Vec<Rc<RefCell<dyn HtmlNode>>>;
    fn append_child(&mut self, child: Rc<RefCell<dyn HtmlNode>>);
}

pub trait HtmlNodeType: Sized + 'static {
    type Host: Host;
    type Attributes;

    fn name() -> &'static str;
    fn new(host: Self::Host, attrs: Self::Attributes) -> Self;
    fn children(&self) -> Vec<Rc<RefCell<dyn HtmlNode>>> {
        vec![]
    }
    fn append_child(&mut self, _child: Rc<RefCell<dyn HtmlNode>>) {
        unimplemented!()
    }
}

pub struct HtmlNodeState<T: HtmlNodeType> {
    state: T,
}

impl<T: HtmlNodeType> HtmlNode for HtmlNodeState<T> {

    fn children(&self) -> Vec<Rc<RefCell<dyn HtmlNode>>> {
        self.state.children()
    }

    fn append_child(&mut self, child: Rc<RefCell<dyn HtmlNode>>) {
        self.state.append_child(child.clone());
    }
}

impl<T: HtmlNodeType> HtmlNodeState<T> {
    pub(crate) fn new(attrs: T::Attributes) -> Rc<RefCell<dyn HtmlNode>> {
        Rc::new(RefCell::new(Self {
            state: T::new(attrs),
        }))
    }
}
