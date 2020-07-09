mod host;

use std::rc::Rc;
use web_sys::{window, Node, EventTarget};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use crate::host::Browser;
use renderer::html::element::HTMLElement;


pub struct App {
    renderer: Rc<Renderer>,
}

impl App {
    pub fn new() -> Self {
        let host = Browser::new("#root");

        Self {
            renderer: Rc::new(Renderer::new(host)),
        }
    }

    pub fn trigger_test(&self) {
        let text = self.renderer.create_text_node("hello");
        let div = self.renderer.create_element::<HTMLDivElement>();

        div.append_child(&text.into());

        self.renderer.root().append_child(&div.into());
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::new();
    let document = web_sys::window().unwrap().document().unwrap();
    let event_target: EventTarget = document.into();
    let handle_keydown = Closure::wrap(Box::new(move || {
        app.trigger_test();
    }) as Box<dyn FnMut()>);

    event_target.add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref());
    handle_keydown.forget();
}
