use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;

pub struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<Fragment>,
}

impl From<Rc<dyn Node>> for Slice {
    fn from(node: Rc<dyn Node>) -> Self {
        Self::new(Rc::new(Fragment::from(node)), 0, 0)
    }
}

impl From<Vec<Rc<dyn Node>>> for Slice {
    fn from(nodes: Vec<Rc<dyn Node>>) -> Self {
        Self::new(Rc::new(Fragment::from(nodes)), 0, 0)
    }
}

impl From<Rc<Fragment>> for Slice {
    fn from(content: Rc<Fragment>) -> Self {
        Self::new( content, 0, 0)
    }
}

impl Slice {
    pub fn new(content: Rc<Fragment>, open_start: usize, open_end: usize) -> Self {
        Self { content, open_start, open_end }
    }

    pub fn empty() -> Self {
        Self::from(vec![])
    }

    pub fn size(&self) -> usize {
        self.content.size()
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
