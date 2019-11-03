mod dom;
mod render;

use wasm_bindgen::prelude::*;
use editor_core::pre::one_test;
use crate::render::render_virtual_node;
use crate::dom::element::Element;

#[wasm_bindgen]
pub fn bootstrap(root: web_sys::Element) -> Result<(), JsValue> {
    let virtual_node = one_test(8, 8).unwrap();

    let mut root_element = Element::from(root);
    let mut content = render_virtual_node(&virtual_node);

    root_element.append_child(&mut content);
    Ok(())
}

#[cfg(test)]
mod test {
    use editor_core::pre::one_test;

    #[test]
    fn test_bootstrap() {
        one_test(8 ,8);
    }
}
