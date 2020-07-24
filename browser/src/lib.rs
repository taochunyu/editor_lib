mod host;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use renderer::Renderer;
use document::Document;
use crate::host::Browser;

#[wasm_bindgen(start)]
pub fn start() {
    let host = Browser::new("#root");
    let renderer = Rc::new(Renderer::new(host));
    let mut doc = Document::new(renderer);

    let document = web_sys::window().unwrap().document().unwrap();
    let event_target: web_sys::EventTarget = document.into();
    let handle_keydown = Closure::wrap(Box::new(move || {
        doc.trigger_test();
    }) as Box<dyn FnMut()>);

    event_target.add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref()).unwrap();
    handle_keydown.forget();
}
