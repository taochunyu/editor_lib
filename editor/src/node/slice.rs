use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;

struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<Fragment>,
}

impl Slice {
    fn open_start(&self) -> usize {
        self.open_start
    }

    fn open_end(&self) -> usize {
        self.open_end
    }

    fn content(&self) -> Rc<Fragment> {
        self.content.clone()
    }
}
