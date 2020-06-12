use wasm_bindgen::prelude::*;
use editor::doc::document::Document;
use editor::n::node::Node;
use editor::n::content::Content;
use js_sys::Array;
use std::rc::Rc;
use std::ops::Deref;
use editor::slice::slice::Slice;

#[wasm_bindgen]
#[derive(Clone)]
pub struct JsNodeObject {
    tag: String,
    children: Vec<JsNodeObject>,
    text: String,
    size: usize,
}

#[wasm_bindgen]
impl JsNodeObject {
    #[wasm_bindgen(getter)]
    pub fn tag(&self) -> String {
        self.tag.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn children(&self) -> Array {
        self.children.clone().into_iter().map(JsValue::from).collect()
    }
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.size
    }
}

fn covert_node_to_js_node(node: &Rc<Node>) -> JsNodeObject {
    let tag = node.node_type().name();
    let size = node.size();
    let (children, text) = match node.content().deref() {
        Content::Nothing => (vec![], String::new()),
        Content::Text(text) => (vec![], String::from(text)),
        Content::Elements(fragment) => (
            fragment.content().into_iter().map(covert_node_to_js_node).collect(),
            String::new(),
        ),
    };

    JsNodeObject { tag, children, text, size }
}

fn diff(before: &Rc<Node>, after: &Rc<Node>) -> Option<usize> {
    let before_length = before.content().count();
    let after_length = after.content().count();
    let length = if before_length > after_length { after_length } else { before_length };

    for i in 0..length {
        if let Ok(b) = before.child(i) {
            if let Ok(a) = after.child(i) {
                if !Rc::ptr_eq(b, a) {
                    return Some(i);
                }
            }
        }
    }

    None
}

#[wasm_bindgen]
pub struct Patch {
    pub position: usize,
    pub size: usize,
    text: String,
}

#[wasm_bindgen]
impl Patch {
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

#[wasm_bindgen]
pub struct Doc {
    doc: Rc<Document>,
}

#[wasm_bindgen]
impl Doc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Doc {
        Doc { doc: Document::new() }
    }

    pub fn get_doc(&self) -> JsNodeObject {
        covert_node_to_js_node(self.doc.root())
    }

    pub fn update_doc(&mut self, from: usize, to: usize, text: &str) -> Patch {
        let new_doc = Rc::new(self.doc.replace(from, to, Slice::text(text)).unwrap());
        let position = diff(self.doc.root(), new_doc.root()).unwrap();

        self.doc = new_doc;

        let node = self.doc.root().child(position).unwrap().child(0).unwrap();
        let text = match node.content().deref() {
            Content::Text(text) => String::from(text),
            _ => String::new(),
        };

        Patch { position, text, size: node.size() }
    }
}
