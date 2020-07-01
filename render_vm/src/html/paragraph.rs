use std::rc::Rc;
use std::cell::RefCell;
use crate::html::{Node, NodeDescription};

pub struct Paragraph {
    children: Vec<Rc<RefCell<dyn Node>>>,
}

impl NodeDescription for Paragraph {
    type Attributes = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn new(attrs: Self::Attributes) -> Result<Self, String> {
        Ok(Paragraph { children: vec![] })
    }

    fn to_instruction(&self) -> Vec<String> {
        vec![String::from(Self::name())]
    }

    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        self.children.clone()
    }

    fn append_child(&mut self, child: Rc<RefCell<dyn Node>>) -> Result<(), String> {
        self.children.push(child);

        Ok(())
    }
}