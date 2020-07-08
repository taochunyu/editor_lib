use std::rc::Rc;
use crate::node::Node;
use crate::node_types::root::Root;
use crate::node::utils::create_element;

pub struct State {
    root_node: Rc<dyn Node>,
    selection: (usize, usize),
}

impl State {
    pub fn new() -> State {
        let root_node = create_element::<Root>((), Some(vec![]));

        Self { root_node, selection: (0, 0) }
    }
}