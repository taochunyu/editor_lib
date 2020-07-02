use std::rc::Rc;
use std::cell::RefCell;
use crate::html::{Node, NodeDescription};

pub struct Div {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl NodeDescription for Div {
    type Attributes = ();

    fn name() -> &'static str {
        "div"
    }

    fn new(_attrs: Self::Attributes) -> Self {
        Div { children: vec![] }
    }

    fn to_instruction(&self) -> Vec<String> {
        vec![String::from(Self::name())]
    }

    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        self.children.clone()
    }

    fn append_child(&mut self, child: Rc<RefCell<dyn Node>>) {
        self.children.push(child);
    }
}