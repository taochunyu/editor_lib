use crate::fragment::fragment::Fragment;
use std::rc::Rc;

pub struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<Fragment>,
}

impl Slice {
    pub fn size(&self) -> usize {
        self.content.size() - self.open_start - self.open_end
    }
}
