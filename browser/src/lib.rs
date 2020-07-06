use std::rc::Rc;
use wasm_bindgen::prelude::*;
use js_sys::*;
use web_sys;
use document::view::View;
use document::node_types::paragraph::Paragraph;
use document::node_types::root::Root;
use document::document::Document;
use renderer::instruction::Instruction;
use std::collections::HashMap;

enum Node {
    Element(web_sys::Element),
    Text(web_sys::Text),
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = stream)]
    fn write(s: Array);
}


#[wasm_bindgen]
pub struct InstructionStream {
    document: web_sys::Document,
    view: Rc<View>,
    map: HashMap<String, Node>,
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

        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let root = Node::Element(document.query_selector("#root").unwrap().unwrap());

        let mut map = HashMap::new();

        map.insert(String::from("0"), root);

        Self { view, document, map }
    }

    pub fn trigger(&mut self) {
        let ins = self.view.ui().flush().iter().map(JsValue::from).collect::<Array>();

        write(ins);
    }

    pub fn trigger_wasm(&mut self) {
        for instruction in self.view.ui().instructions().borrow().iter() {
            match instruction {
                Instruction::Append(desc) => {
                    let id = desc.get(0).unwrap();
                    let parent_id = desc.get(1).unwrap();
                    let parent = self.map.get(parent_id);
                    let node = self.map.get(id);

                    if let (Some(parent), Some(node)) = (parent, node) {
                        if let Node::Element(parent) = parent {

                            match node {
                                Node::Element(node) => parent.append_child(&node),
                                Node::Text(node) => parent.append_child(&node),
                            };
                        };
                    };
                },
                Instruction::Create(desc) => {
                    let id = desc.get(0).unwrap();
                    let tag_name = desc.get(2).unwrap();
                    let node = match tag_name.as_str() {
                        "text" => {
                            let content = desc.get(3).unwrap();
                            let text = self.document.create_text_node(content.as_str());

                            Node::Text(text)
                        }
                        "paragraph" => Node::Element(self.document.create_element("p").unwrap()),
                        _ => Node::Element(self.document.create_element("div").unwrap()),
                    };

                    self.map.insert(String::from(id), node);
                }
            }
        }
    }

    pub fn trigger_test(&self) {
        let root = self.document.query_selector("#root").unwrap().unwrap();
        let mut html = String::new();

        for _ in 0..10000 {
            html.push_str("<p>hello</p>");
        }

        root.set_inner_html(html.as_str());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}

