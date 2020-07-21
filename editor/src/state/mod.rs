mod text_selection;
mod node_selection;

use std::rc::Rc;
use crate::Doc;
use crate::state::selection::Selection;

mod transaction;
mod selection;

pub struct State {
    doc: Doc,
    selection: Rc<dyn Selection>,
}

impl State {
    pub fn new(doc: Doc, selection: Rc<dyn Selection>) -> Self {
        Self { doc, selection }
    }

    pub fn doc(&self) -> Doc {
        self.doc.clone()
    }

    pub fn selection(&self) -> Rc<dyn Selection> {
        self.selection.clone()
    }
}


#[cfg(test)]
mod test {
    use crate::test::tools::{create_doc, create_empty_slice};
    use crate::state::State;
    use crate::state::text_selection::TextSelection;
    use std::rc::Rc;
    use crate::state::transaction::Transaction;

    #[test]
    fn replace_selection_works() {
        let doc = create_doc();
        let slice = create_empty_slice();
        let selection = Rc::new(TextSelection::new(doc.clone(), 3, 4).unwrap());
        let state = State::new(doc, selection);
        let mut transaction = Transaction::new(&state);

        transaction.replace_selection(slice);

        println!("{}", transaction.doc().serialize());
    }
}