mod node_view_desc;
mod view_desc;
mod updater;

use std::rc::Rc;
use std::cell::RefCell;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use renderer::html::node::HTMLNode;

use crate::node::Node;
use crate::state::State;
use crate::{Doc, Position};
use crate::state::transaction::Transaction;
use crate::view::node_view_desc::NodeViewDesc;
use crate::view::view_desc::ViewDesc;

pub struct View {
    renderer: Rc<Renderer>,
    state: State,
    doc_view: Rc<dyn ViewDesc>
}

impl View {
    pub fn new(renderer: Rc<Renderer>, dom: HTMLDivElement, doc: Doc) -> Self {
        let state = State::new(doc.clone());
        let doc_view = NodeViewDesc::new(
            None,
            doc,
            dom.clone().into(),
            Some(dom.clone().into()),
            0,
            renderer.clone(),
        );

        Self {
            renderer,
            state,
            doc_view,
        }
    }

    pub fn renderer(&self) -> Rc<Renderer> {
        self.renderer.clone()
    }

    pub fn dispatch(&mut self, transaction: &Transaction) {
        self.state = self.state.apply(transaction);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {}
}