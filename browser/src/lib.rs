mod host;

use web_sys::{window, Node, EventTarget};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use renderer::Renderer;
use renderer::html::div::HtmlDivElement;
use crate::host::Browser;
use document::node_types::paragraph::Paragraph;
use document::node_types::root::Root;
use document::view::View;
use document::node::utils::{create_element, create_text};
use std::rc::Rc;


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

    pub fn trigger_test_doc(&self) {
        let div = self.renderer.create_element::<HtmlDivElement>();

        self.renderer.root().append_child(&div.clone().into());

        let mut content = vec![];

        for _ in 0..1 {
            let hello = create_text("hello, world");
            let paragraph = create_element::<Paragraph>(
                (),
                Some(vec![hello]),
            );

            content.push(paragraph);
        }

        let doc = create_element::<Root>((), Some(content));
        let view = View::new(self.renderer.clone(), div, doc);

        view.init();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let app = App::new();
    let document = web_sys::window().unwrap().document().unwrap();
    let event_target: EventTarget = document.into();
    let handle_keydown = Closure::wrap(Box::new(move || {
        app.trigger_test_doc();
    }) as Box<dyn FnMut()>);

    event_target.add_event_listener_with_callback("keydown", handle_keydown.as_ref().unchecked_ref());
    handle_keydown.forget();
}
