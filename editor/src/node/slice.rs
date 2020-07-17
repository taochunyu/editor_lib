use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;

pub struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<Fragment>,
}

impl Slice {
    pub fn new(open_start: usize, open_end: usize, content: Vec<Rc<dyn Node>>) -> Self {
        Self {
            open_start,
            open_end,
            content: Rc::new(Fragment::from(content)),
        }
    }

    pub fn open_start(&self) -> usize {
        self.open_start
    }

    pub fn open_end(&self) -> usize {
        self.open_end
    }

    pub fn content(&self) -> Rc<Fragment> {
        self.content.clone()
    }
}