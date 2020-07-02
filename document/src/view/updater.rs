use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::view::node_view::NodeView;
use crate::node::Node;
use crate::view::View;

pub struct Updater {
    top: Rc<RefCell<NodeView>>,
    index: usize,
    changed: bool,
}

impl Updater {
    pub(crate) fn new(top: Rc<RefCell<NodeView>>) -> Updater {
        Self { top, index: 0, changed: false }
    }

    pub(crate) fn add_node(&mut self, node: Rc<dyn Node>, view: Rc<View>, offset: usize) {
        let node_view = NodeView::create(node, self.top.clone(), view, offset);

        self.top.borrow_mut().insert_child(self.index, node_view);
        self.index += 1;
        self.changed = true;
    }
}
