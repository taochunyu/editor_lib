use crate::node::node_content::NodeContent;
use std::rc::Rc;

pub struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<NodeContent>,
}

impl Slice {
    pub fn size(&self) -> usize {
        self.content.size() - self.open_start - self.open_end
    }
    pub fn open_start(&self) -> usize {
        self.open_start
    }
    pub fn open_end(&self) -> usize { self.open_end }
}
