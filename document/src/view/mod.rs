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
    state: State,
    test_count: u64,
}

pub struct View<H: Host> {
    renderer: Rc<Renderer<H>>,
    root_html_element: Rc<HtmlElement<H>>,
    // root_node_view: Rc<RefCell<NodeView>>,
    state_cell: RefCell<StateCell>,
}

impl<H: Host> View<H> {
    pub fn new(renderer: Rc<Renderer<H>>, root_html_element: Rc<HtmlElement<H>>, state: State) -> Rc<Self> {
        Rc::new(Self {
            renderer,
            root_html_element,
            state_cell: RefCell::new(StateCell {
                state,
                test_count: 0,
            }),
        })
    }

    pub(crate) fn renderer(&self) -> Rc<Renderer<H>> {
        self.renderer.clone()
    }

    pub fn dispatch(&self) {
        let state_cell = self.state_cell.borrow_mut();

        state_cell.test_count += 1;
    }
}
