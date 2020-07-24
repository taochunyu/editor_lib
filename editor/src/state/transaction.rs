use std::rc::Rc;
use crate::transform::Transform;
use crate::state::State;
use crate::Doc;
use crate::node::slice::Slice;
use crate::state::selection::Selection;

pub struct Transaction {
    transform: Transform,
    selection: Option<Rc<dyn Selection>>,
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

    pub fn selection(&self) -> Option<Rc<dyn Selection>> {
        self.selection.clone()
    }

    pub fn set_selection(&mut self, selection: Option<Rc<dyn Selection>>) -> &mut Self {
        self.selection = selection;

        self
    }

    pub fn replace_selection(&mut self, slice: Slice) -> &mut Self {
        if let Some(selection) = &self.selection {
            selection.replace(&mut self.transform, slice);
        }

        self
    }
}
