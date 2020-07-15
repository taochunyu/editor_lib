use std::rc::Rc;
use std::cell::RefCell;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::node::Node;
use crate::view::View;
use crate::view::updater::Updater;

pub struct NodeView {
    node: Rc<dyn Node>,
    parent: Option<Rc<RefCell<NodeView>>>,
    children: Vec<Rc<RefCell<NodeView>>>,
    dom: HTMLNode,
    content_dom: Option<HTMLElement>,
}

impl NodeView {
    pub(crate) fn new(
        node: Rc<dyn Node>,
        parent: Option<Rc<RefCell<NodeView>>>,
        dom: HTMLNode,
        content_dom: Option<HTMLElement>,
    ) -> Rc<RefCell<NodeView>> {
        Rc::new(RefCell::new(Self {
            node,
            parent,
            children: vec![],
            dom,
            content_dom,
        }))
    }

    pub(crate) fn create(
        node: Rc<dyn Node>,
        parent: Rc<RefCell<NodeView>>,
        view: Rc<View>,
    ) -> Rc<RefCell<NodeView>> {
        let (dom, content_dom) = node.clone().render(view.clone());

        Self::new(node, Some(parent), dom, content_dom)
    }

    pub(crate) fn dom(&self) -> HTMLNode {
        self.dom.clone()
    }

    pub(crate) fn content_dom(&self) -> Option<HTMLElement> {
        self.content_dom.clone()
    }

    pub(crate) fn insert_child(&mut self, index: usize, child: Rc<RefCell<NodeView>>) {
        self.children.insert(index, child);
    }

    pub(crate) fn update_children(node_view: Rc<RefCell<NodeView>>, view: Rc<View>, offset: usize) {
        let mut updater = Updater::new(node_view.clone());
        let mut top = node_view.borrow_mut();

        if let Some(children) = top.node.children() {
            for child in children.content() {
                updater.add_node(child, view.clone(), offset, &mut top);
            }
        }
    }
}
