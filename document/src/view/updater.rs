use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use crate::view::node_view::NodeView;
use crate::node::Node;
use crate::view::View;
use renderer::host::Host;

pub struct Updater<H: Host> {
    top: Rc<RefCell<NodeView<H>>>,
    index: usize,
    changed: bool,
}

impl<H: Host> Updater<H> {
    pub(crate) fn new(top: Rc<RefCell<NodeView<H>>>) -> Updater<H> {
        Self { top, index: 0, changed: false }
    }

    pub(crate) fn add_node(&mut self, node: Rc<dyn Node>, view: Rc<View<H>>, offset: usize, parent: &mut NodeView<H>) {
        let node_view = NodeView::create(node, self.top.clone(), view.clone());

        node_view.borrow_mut().update_children(view.clone(), offset);

        parent.insert_child(self.index, node_view.clone());

        if let Some(content_dom) = parent.content_dom() {
            content_dom.append_child(node_view.borrow().dom().as_ref());
        }

        self.index += 1;
        self.changed = true;
    }
}
