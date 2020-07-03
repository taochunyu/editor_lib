mod updater;
mod node_view;

use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use render_vm::ui::UI;
use render_vm::html;
use render_vm::html::div::Div;
use crate::view::node_view::NodeView;
use crate::node::Node;
use render_vm::html::operation::append_child;

struct NodeViewTree {
    root: Option<Rc<RefCell<NodeView>>>,
}

pub struct View {
    ui: RefCell<UI>,
    dom: Rc<RefCell<dyn html::Node>>,
    root_node: RefCell<Rc<dyn Node>>,
    node_view_tree: RefCell<NodeViewTree>,
}

impl View {
    pub fn new(root_node: Rc<dyn Node>) -> Rc<Self> {
        let ui = RefCell::new(UI::new());
        let dom = ui.borrow_mut().create_element::<Div>(());
        let view = Rc::new(View {
            ui,
            dom,
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

        // append_child(view.dom.clone(), root_node_view.borrow().dom());

        view
    }

    pub fn ui(&self) -> RefMut<UI>{
        self.ui.borrow_mut()
    }
}
