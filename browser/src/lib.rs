mod host;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use renderer::Renderer;
use document::Document;
use crate::host::host::Browser;
use host::dom_observer::DOMObserver;


#[wasm_bindgen(start)]
pub fn start() {
    let host = Browser::new("#editor");
    let renderer = Rc::new(Renderer::new(host));
    let mut doc = Document::new(renderer.clone());

    let document = web_sys::window().unwrap().document().unwrap();
    let node = document.query_selector("#editor").unwrap().unwrap().into();
    let dom_observer = DOMObserver::new(node);

    dom_observer.start();

    // let document = web_sys::window().unwrap().document().unwrap();
    // let event_target: web_sys::EventTarget = document.clone().into();
    // let handle_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
    //     event.prevent_default();
    //     doc.trigger_test();
    // }) as Box<dyn FnMut(_)>);
    //
    // event_target.add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref()).unwrap();
    // handle_keydown.forget();
}
