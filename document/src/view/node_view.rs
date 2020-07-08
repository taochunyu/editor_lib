use std::rc::Rc;
use std::cell::RefCell;
use renderer::html;
use renderer::host::Host;
use crate::node::Node;
use crate::view::View;
use crate::view::updater::Updater;
use renderer::html::{HtmlNode, HtmlElement};

pub struct NodeView<H: Host> {
    node: Rc<dyn Node>,
    parent: Option<Rc<RefCell<NodeView<H>>>>,
    children: Vec<Rc<RefCell<NodeView<H>>>>,
    dom: Rc<HtmlNode<H>>,
    content_dom: Option<Rc<HtmlElement<H>>>,
}

impl<H: Host> NodeView<H> {
    pub(crate) fn new(
        node: Rc<dyn Node>,
        parent: Option<Rc<RefCell<NodeView<H>>>>,
        dom: Rc<HtmlNode<H>>,
        content_dom: Option<Rc<HtmlElement<H>>>,
    ) -> Rc<RefCell<NodeView<H>>> {
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
        parent: Rc<RefCell<NodeView<H>>>,
        view: Rc<View<H>>,
    ) -> Rc<RefCell<NodeView<H>>> {
        let (dom, content_dom) = node.clone().render(view.clone());

        Self::new(node, Some(parent), dom, content_dom)
    }

    pub(crate) fn insert_child(&mut self, index: usize, child: Rc<RefCell<NodeView<H>>>) {
        self.children.insert(index, child);
    }

    pub(crate) fn update_children(&mut self, view: Rc<View<H>>, offset: usize) {
        let mut updater = Updater::new(self.node.clone());
        let mut top = self;

        if let Some(children) = top.node.children() {
            for child in children.content() {
                updater.add_node(child, view.clone(), offset, &mut top);
            }
        }
    }

    pub(crate) fn dom(&self) -> Rc<HtmlNode<H>> {
        self.dom.clone()
    }

    pub(crate) fn content_dom(&self) -> Option<Rc<HtmlElement<H>>> {
        self.content_dom.clone()
    }
}
