mod updater;
mod node_view;

use std::rc::Rc;
use std::cell::RefCell;
use renderer::Renderer;
use renderer::host::Host;
use renderer::html::HtmlElement;
use crate::node::Node;
use crate::state::State;
use crate::view::node_view::NodeView;

struct StateCell {
    // state: State,
    test_count: u64,
    root_node: Rc<dyn Node>,
}

pub struct View {
    renderer: Rc<Renderer>,
    root_html_element: HtmlElement,
    root_node_view: Rc<RefCell<NodeView>>,
    state_cell: RefCell<StateCell>,
}

impl View {
    pub fn new(renderer: Rc<Renderer>, root_html_element: HtmlElement, root_node: Rc<dyn Node>) -> Rc<Self> {
        let root_node_view = NodeView::new(
            root_node.clone(),
            None,
            root_html_element.clone().into(),
            Some(root_html_element.clone()),
        );

        Rc::new(Self {
            renderer,
            root_html_element,
            root_node_view,
            state_cell: RefCell::new(StateCell {
                // state,
                test_count: 0,
                root_node,
            }),
        })
    }

    pub fn init(self: Rc<Self>) {
        NodeView::update_children(self.root_node_view.clone(), self, 0);
    }

    pub(crate) fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn dispatch(&self) {
        let mut state_cell = self.state_cell.borrow_mut();

        state_cell.test_count += 1;
    }
}
