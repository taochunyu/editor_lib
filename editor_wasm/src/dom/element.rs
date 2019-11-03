use wasm_bindgen::JsCast;

pub struct Element {
    element: web_sys::Element,
}

impl From<web_sys::Element> for Element {
    fn from(element: web_sys::Element) -> Element {
        Element { element }
    }
}

impl From<Element> for web_sys::Node {
    fn from(el: Element) -> web_sys::Node {
        el.element.into()
    }
}

pub enum ElementType {
    Div,
    P,
    Span,
    Unknown,
}

impl Element {
    pub fn create_element(element_type: ElementType) -> Option<Element> {
        let tag = match element_type {
            ElementType::Div => "div",
            ElementType::P => "p",
            ElementType::Span => "span",
            ElementType::Unknown => "template",
        };

        if let Some(el) = web_sys::window()?.document()?.create_element(tag).ok() {
            Some(el.into())
        } else {
            None
        }
    }

    pub fn set_inner_html(&mut self, html: String) {
        self.element.set_inner_html(&html);
    }

    pub fn append_child(&mut self, child: &mut Element) {
        if let Some(node) = self.element.dyn_ref::<web_sys::Node>() {
            if let Some(child_node) = child.element.dyn_ref::<web_sys::Node>() {
                node.append_child(child_node).unwrap();
            }
        }
    }
}
