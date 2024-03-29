mod text_view_desc;
mod node_view_desc;
pub mod view_desc;
mod updater;

use std::rc::Rc;
use std::cell::RefCell;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use renderer::html::node::HTMLNode;
use renderer::host::ExtraInfo;
use crate::node::Node;
use crate::state::State;
use crate::{Doc, Position};
use crate::state::transaction::Transaction;
use crate::view::node_view_desc::NodeViewDesc;
use crate::view::view_desc::ViewDesc;
use std::any::Any;

#[derive(Clone)]
pub struct TypedExtraInfo {
    view_desc: Rc<dyn ViewDesc>,
}

impl TypedExtraInfo {
    pub fn get_view_desc(&self) -> Rc<dyn ViewDesc> {
        self.view_desc.clone()
    }
}

impl TypedExtraInfo {
    pub fn new(view_desc: Rc<dyn ViewDesc>) -> Box<dyn ExtraInfo> {
        Box::new(Self { view_desc })
    }
}

impl ExtraInfo for TypedExtraInfo {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct View {
    renderer: Rc<Renderer>,
    doc_view: Rc<dyn ViewDesc>
}

impl View {
    pub fn new(renderer: Rc<Renderer>, dom: HTMLDivElement, state: &State) -> Self {
        let doc_view = NodeViewDesc::new(
            None,
            state.doc(),
            dom.clone().into(),
            Some(dom.clone().into()),
            0,
            renderer.clone(),
        );

        Self {
            renderer,
            doc_view,
        }
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn render(&mut self, state: &State) {
        if !self.doc_view.clone().update(state.doc()) {
            self.doc_view = NodeViewDesc::new(
                None,
                state.doc(),
                self.doc_view.dom(),
                self.doc_view.content_dom(),
                0,
                self.renderer(),
            );
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}
