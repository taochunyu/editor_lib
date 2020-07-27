pub mod text_selection;
pub mod node_selection;
pub mod transaction;
pub mod selection;

use std::rc::Rc;
use crate::Doc;
use crate::state::selection::Selection;
use crate::state::transaction::Transaction;

pub struct State {
    doc: Doc,
    selection: Option<Rc<dyn Selection>>,
}

impl State {
    pub fn new(doc: Doc) -> Self {
        Self { doc, selection: None }
    }

    pub fn doc(&self) -> Doc {
        self.doc.clone()
    }

    pub fn selection(&self) -> Option<Rc<dyn Selection>> {
        self.selection.clone()
    }

    pub fn create_transaction(&self) -> Transaction {
        Transaction::new(&self)
    }

    pub fn apply(&self, transaction: &Transaction) -> State {
        State {
            doc: transaction.doc(),
            selection: transaction.selection(),
        }
    }
}


#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::test::tools::{create_doc, create_empty_slice};
    use crate::state::State;
    use crate::state::text_selection::TextSelection;
    use crate::state::transaction::Transaction;

    #[test]
    fn replace_selection_works() {
        let doc = create_doc();
        let state = State::new(doc.clone());
        let slice = create_empty_slice();
        let selection = Rc::new(TextSelection::new(doc.clone(), 3, 4).unwrap());
        let mut transaction = Transaction::new(&state);

        transaction.set_selection(Some(selection)).replace_selection(slice);

        println!("{}", transaction.doc().serialize());
    }
}
