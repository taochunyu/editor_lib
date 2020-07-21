use std::rc::Rc;
use crate::transform::Transform;
use crate::state::State;
use crate::Doc;
use crate::node::slice::Slice;
use crate::state::selection::Selection;

pub struct Transaction {
    transform: Transform,
    selection: Rc<dyn Selection>,
}

impl Transaction {
    pub fn new(state: &State) -> Self {
        Self {
            transform: Transform::new(state.doc()),
            selection: state.selection(),
        }
    }

    pub fn doc(&self) -> Doc {
        self.transform.doc()
    }

    pub fn replace_selection(&mut self, slice: Slice) -> &mut Self {
        self.selection.replace(&mut self.transform, slice);

        self
    }
}
