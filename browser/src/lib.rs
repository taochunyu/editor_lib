use std::rc::Rc;
use wasm_bindgen::prelude::*;
use js_sys::*;
use document::view::View;
use document::node_types::paragraph::Paragraph;
use document::node_types::root::Root;
use document::document::Document;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = stream)]
    fn write(s: Array);
}


#[wasm_bindgen]
pub struct InstructionStream {
    view: Rc<View>
}

#[wasm_bindgen]
impl InstructionStream {
    #[wasm_bindgen(constructor)]
    pub fn new() -> InstructionStream {
        let mut children = vec![];

        for _ in 0..10000 {
            let hello = Document::create_text("hello");
            let paragraph = Document::create_element::<Paragraph>(
                (),
                Some(vec![hello]),
            );

            children.push(paragraph);
        }

        let doc = Document::create_element::<Root>((), Some(children));

        let view = View::new(doc);

        Self { view }
    }

    pub fn trigger(&mut self) {
        let ins = self.view.ui().flush().iter().map(JsValue::from).collect::<Array>();

        write(ins);
    }
}

