use std::rc::Rc;
use std::cell::RefCell;
use render_vm::html;
use crate::node::Node;
use crate::view::View;
use crate::view::updater::Updater;
use render_vm::html::operation::append_child;

pub struct NodeView {
    node: Rc<dyn Node>,
    parent: Option<Rc<RefCell<NodeView>>>,
    children: Vec<Rc<RefCell<NodeView>>>,
    dom: Rc<RefCell<dyn html::Node>>,
    content_dom: Option<Rc<RefCell<dyn html::Node>>>,
}

impl NodeView {
    pub(crate) fn new(
        node: Rc<dyn Node>,
        parent: Option<Rc<RefCell<NodeView>>>,
        dom: Rc<RefCell<dyn html::Node>>,
        content_dom: Option<Rc<RefCell<dyn html::Node>>>,
        view: Rc<View>,
        offset: usize,
    ) -> Rc<RefCell<NodeView>> {
        let children = vec![];
        let node_view = Rc::new(RefCell::new(Self {
            node,
            parent,
            children,
            dom,
            content_dom,
        }));

        Self::update_children(node_view.clone(), view.clone(), offset);

        node_view
    }

    pub(crate) fn create(
        node: Rc<dyn Node>,
        parent: Rc<RefCell<NodeView>>,
        view: Rc<View>,
        offset: usize,
    ) -> Rc<RefCell<NodeView>> {
        let (dom, content_dom) = node.clone().render(view.clone());

        append_child(parent.borrow().dom.clone(), dom.clone());

        Self::new(node, Some(parent), dom, content_dom, view, offset)
    }

    pub(crate) fn insert_child(&mut self, index: usize, child: Rc<RefCell<NodeView>>) {
        self.children.insert(index, child);
    }

    fn update_children(node: Rc<RefCell<NodeView>>, view: Rc<View>, offset: usize) {
        let mut updater = Updater::new(node.clone());

        if let Some(children) = node.borrow().node.children() {
            for child in children.content() {
                updater.add_node(child, view.clone(), offset);
            }
        }
    }
}
