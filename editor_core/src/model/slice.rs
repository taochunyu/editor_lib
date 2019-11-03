use crate::model::fragment;
use crate::model::fragment::Fragment;
use std::rc::Rc;

pub struct Slice {
    pub content: Rc<Fragment>,
    pub open_start: usize,
    pub open_end: usize,
}

impl Slice {
    pub fn new(content: Rc<Fragment>, open_start: usize, open_end: usize) -> Slice {
        Slice {
            content,
            open_start,
            open_end,
        }
    }
    pub fn size(&self) -> usize {
        self.content.size
    }
}
