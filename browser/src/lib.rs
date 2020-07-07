mod host;

use web_sys::{window, Node};
use wasm_bindgen::prelude::*;
use renderer::Renderer;
use renderer::html::div::Div;
use crate::host::Browser;


pub struct App {
    renderer: Renderer<Browser>,
    root: Node,
}

impl App {
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

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::new();

    app.trigger_test();
}

#[cfg(test)]
mod tests {
    use crate::start;

    #[test]
    fn it_works() {
    }
}

