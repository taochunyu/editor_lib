pub mod host;

use std::rc::Rc;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use editor::node::slice::Slice;
use editor::node::utils::{create_text, create_element};
use editor::node_types::paragraph::{Paragraph, ParagraphAttributes};
use editor::node_types::root::{Root, RootAttributes};
use editor::view::View;
use editor::state::State;
use editor::state::text_selection::TextSelection;
use editor::state::transaction::Transaction;

pub struct Document {
    // view: View,
}

impl Document {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let div = renderer.create_element::<HTMLDivElement>();

        renderer.root().append_child(&div.clone().into());

        let mut content = vec![];

        for _ in 0..10000 {
            let hello = create_text("helloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworldhelloworld");
            let paragraph = create_element::<Paragraph>(
                ParagraphAttributes::new(),
                Some(vec![hello]),
            );

            content.push(paragraph);
        }

        let doc = create_element::<Root>(RootAttributes::new(), Some(content));
        let state = State::new(doc);

        Self {
            // view: View::new(renderer, div, state),
        }
    }

    pub fn trigger_test(&mut self) {
        // let slice = self.view.state().doc().slice(0, 201).unwrap();
        // let selection = Rc::new(TextSelection::new(self.view.state().doc(), 0, 0).unwrap());
        // let mut transaction = self.view.state().create_transaction();
        //
        // transaction.set_selection(Some(selection)).replace_selection(slice);

        // self.view.dispatch(&transaction)
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use renderer::Renderer;
    use editor::view::View;
    use crate::Document;
    use crate::host::TestHost;

    #[test]
    fn doc_init_works() {
        let test_host = TestHost::new();
        let renderer = Rc::new(Renderer::new(test_host));

        Document::new(renderer);
    }

    #[test]
    fn doc_test_works() {
        let test_host = TestHost::new();
        let renderer = Rc::new(Renderer::new(test_host));

        let mut doc = Document::new(renderer);

        doc.trigger_test();
    }
}
