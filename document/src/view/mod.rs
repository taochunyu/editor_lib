mod updater;
mod node_view;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use renderer::{html, Renderer};
use renderer::html::div::Div;
use renderer::html::operation::append_child;
use renderer::host::Host;
use renderer::html::HtmlNode;
use crate::view::node_view::NodeView;
use crate::node::Node;

pub struct View<H: Host> {
    renderer: Renderer<H>,
    root_node: RefCell<Rc<dyn Node>>,
    root_html_node: HtmlNode<H>,
}

impl View {
    pub fn new(root_node: Rc<dyn Node>) -> Rc<Self> {
        let ui = RefCell::new(UI::new());
        let ui_root = ui.borrow().root();
        let dom = ui.borrow_mut().create_element::<Div>(());

        let view = Rc::new(View {
            ui,
            dom: dom.clone(),
            root_node: RefCell::new(root_node.clone()),
            node_view_tree: RefCell::new(NodeViewTree { root: None }),
        });
        let root_node_view = NodeView::new(
            root_node,
            None,
            view.dom.clone(),
            Some(view.dom.clone()),
            view.clone(),
            0,
        );

        view.clone().node_view_tree.borrow_mut().root = Some(root_node_view.clone());

        append_child(ui_root, dom.clone());

        view
    }

    pub fn ui(&self) -> RefMut<UI>{
        self.ui.borrow_mut()
    }
}
