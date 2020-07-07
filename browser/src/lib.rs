mod host;

use web_sys::{window, Node};
use wasm_bindgen::prelude::*;
use renderer::Renderer;
use crate::host::Browser;
use renderer::html::div::Div;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = stream)]
    fn write(s: &str);
}


#[wasm_bindgen]
pub struct App {
    renderer: Renderer<Browser>,
    root: Node,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let host = Browser::new("#root");
        let document = web_sys::window().unwrap().document().unwrap();
        let root: Node = document.query_selector("#root").unwrap().unwrap().into();

        Self {
            root,
            renderer: Renderer::new(host),
        }
    }

    pub fn trigger_test(&self) {
        let div = self.renderer.create_element::<Div>();

        self.renderer.root().append_child(div);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}

