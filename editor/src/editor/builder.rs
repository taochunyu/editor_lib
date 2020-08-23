use std::rc::Rc;
use std::cell::RefCell;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use crate::Doc;
use crate::editor::Editor;
use crate::state::State;
use crate::view::View;
use crate::schema::Schema;
use crate::node_types::paragraph::{Paragraph, ParagraphAttributes};
use crate::node_types::root::Root;

pub struct EditorBuilder {
    renderer: Rc<Renderer>,
    dom: HTMLDivElement,
    schema: Schema,
    doc: Doc,
}

impl EditorBuilder {
    pub fn new(renderer: Rc<Renderer>, dom: HTMLDivElement, schema: Schema, doc: Doc) -> Self {
        Self {
            renderer,
            dom,
            schema,
            doc,
        }
    }

    pub fn build(self) -> Rc<RefCell<Editor>> {
        let state = State::new(self.doc.clone());
        let view = View::new(self.renderer.clone(), self.dom.clone(), &state);

        Rc::new(RefCell::new(Editor::new(self.schema, state, view)))
    }

    pub fn empty_doc(schema: &Schema) -> Doc {
        let text = schema.create_text_node("");
        let p = schema.create_node::<Paragraph>(
            Rc::new(ParagraphAttributes::new()),
            Some(vec![text]),
        );

        schema.create_node::<Root>(Rc::new(()), Some(vec![p]))
    }
}

impl Editor {
    pub fn builder(renderer: Rc<Renderer>, dom: HTMLDivElement, schema: Schema, doc: Doc) -> EditorBuilder {
        EditorBuilder::new(renderer, dom, schema, doc)
    }
}
