use crate::state::selection::Selection;
use std::rc::Rc;

pub struct State {
    document: Rc<dyn TreeNode>,
    selection: Selection,
}

impl State {}
