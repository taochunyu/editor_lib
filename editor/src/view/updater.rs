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

    pub(crate) fn add_node(&mut self, node: Rc<dyn Node>, view: Rc<View>, offset: usize, parent: &mut RefMut<NodeView>) {
        let node_view = NodeView::create(node, self.top.clone(), view.clone());

        NodeView::update_children(node_view.clone(), view.clone(), offset);

        parent.insert_child(self.index, node_view.clone());

        if let Some(content_dom) = parent.content_dom() {
            content_dom.append_child(&node_view.borrow().dom());
        }

        self.index += 1;
        self.changed = true;
    }
}
