use wasm_bindgen::prelude::*;
use editor::view::TypedExtraInfo;
use std::rc::Rc;
use renderer::host::ExtraInfo;

pub const EXTRA_INFO_NAME: &str = "pointer";

#[wasm_bindgen(inline_js = "export const a=(a,b)=>a[b]||null")]
extern "C" {
    fn a(node: &web_sys::Node, extra_info_name: &str) -> Option<DOMPointer>;
}

#[wasm_bindgen]
pub struct DOMPointer {
    extra_info: Box<dyn ExtraInfo>,
}

impl DOMPointer {
    fn get_extra_info(&self) -> TypedExtraInfo {
        self.extra_info.as_any().downcast_ref::<TypedExtraInfo>().unwrap().clone()
    }
}

impl From<Box<dyn ExtraInfo>> for DOMPointer {
    fn from(extra_info: Box<dyn ExtraInfo>) -> Self {
        Self {
            extra_info
        }
    }
}

impl DOMPointer {
    pub fn set(node: &web_sys::Node, extra_info: Box<dyn ExtraInfo>) {
        let pointer = DOMPointer::from(extra_info);

        Self::set_on_dom(node, pointer);
    }

    pub fn get(node: &web_sys::Node) -> Option<TypedExtraInfo> {
        a(node, EXTRA_INFO_NAME).map(|pointer| {
            let extra_info = pointer.get_extra_info();

            Self::set_on_dom(&node, pointer);

            extra_info
        })
    }

    fn set_on_dom(node: &web_sys::Node, pointer: Self) {
        js_sys::Reflect::set(node, &EXTRA_INFO_NAME.into(), &pointer.into());
    }
}
