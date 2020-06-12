use wasm_bindgen::prelude::*;
use js_sys::*;
use ui::ui::UI;
use ui::element::rectangular::{RectangularProps, Rectangular};
use std::rc::Rc;
use ui::element::text::{TextProps, Text};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = stream)]
    fn write(s: Array);
}


    #[wasm_bindgen]
pub struct InstructionStream {
    ui: UI,
}

#[wasm_bindgen]
impl InstructionStream {
    #[wasm_bindgen(constructor)]
    pub fn new() -> InstructionStream {
        InstructionStream {
            ui: UI::new().unwrap(),
        }
    }

    fn t(&mut self) -> Result<Vec<String>, String>{
        let rect_props = RectangularProps { width: 200, height: 100 };
        let rect = self.ui.create_element::<Rectangular>(rect_props)?;

        let text_props = TextProps { text: String::from("hello world") };
        let text = self.ui.create_element::<Text>(text_props)?;

        self.ui.root_element.borrow_mut().append_child(Rc::clone(&rect))?;

        rect.borrow_mut().append_child(text)?;

        Ok(self.ui.flush())
    }

    pub fn trigger(&mut self) {
        let ins: Array = self.t().unwrap().iter().map(JsValue::from).collect();

        write(ins);
    }
}

