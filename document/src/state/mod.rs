use std::rc::Rc;
use crate::node::{Node, create_node};
use crate::node_types::root::Root;

pub struct State {
    root_node: Rc<dyn Node>,
    selection: (usize, usize),
}

impl State {
    pub fn new() -> State {
        let root_node = create_node::<Root>((), Some(vec![]));

        Self { root_node, selection: (0, 0) }
    }
}