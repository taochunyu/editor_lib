use std::rc::Rc;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use editor::node::slice::Slice;
use editor::node_types::paragraph::{Paragraph, ParagraphAttributes};
use editor::node_types::root::{Root};
use editor::view::View;
use editor::state::State;
use editor::state::text_selection::TextSelection;
use editor::state::transaction::Transaction;
use editor::editor::Editor;
use std::cell::RefCell;
use editor::schema::Schema;
use editor::Doc;

pub struct Document {
    editor: Rc<RefCell<Editor>>,
}

impl Document {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let dom = Self::create_dom(renderer.clone());
        let schema= Self::create_schema();
        let doc = Self::create_doc(&schema);
        let editor = Editor::builder(renderer, dom, schema, doc).build();

        Self { editor }
    }

    pub fn editor(&self) -> Rc<RefCell<Editor>> {
        self.editor.clone()
    }

    pub fn trigger_test(&mut self) {
        // let slice = self.editor.state().doc().slice(0, 201).unwrap();
        // let selection = Rc::new(TextSelection::new(self.editor.state().doc(), 0, 0).unwrap());
        // let mut transaction = self.editor.create_transaction();
        //
        // transaction.set_selection(Some(selection)).replace_selection(slice);
        //
        // self.editor.dispatch(&transaction)
    }

    fn create_dom(renderer: Rc<Renderer>) -> HTMLDivElement {
        let dom = renderer.create_element::<HTMLDivElement>();

        renderer.root().append_child(&dom.clone().into());

        dom
    }

    fn create_schema() -> Schema {
        let mut schema = Schema::new();

        schema.register_node_type::<Root>();
        schema.register_node_type::<Paragraph>();

        schema
    }

    fn create_doc(schema: &Schema) -> Doc {
        let mut content = vec![];

        for _ in 0..5 {
            let hello = schema.create_text_node("hello, world! hello, world!");
            let paragraph = schema.create_node::<Paragraph>(
                Rc::new(ParagraphAttributes::new()),
                Some(vec![hello]),
            );

            content.push(paragraph);
        }

        schema.create_node::<Root>(Rc::new(()), Some(content))
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
