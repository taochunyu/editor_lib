use std::rc::Rc;
use std::cell::{RefMut, RefCell, Ref};
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::node::Node;
use crate::Position;

pub trait ViewDesc {
    fn parent(&self) -> Option<Rc<dyn ViewDesc>>;
    fn children(&self) -> Ref<Vec<Rc<dyn ViewDesc>>>;
    fn node(&self) -> Rc<dyn Node>;
    fn dom(&self) -> HTMLNode;
    fn content_dom(&self) -> Option<HTMLElement>;
    fn size(&self) -> usize;
    fn update(self: Rc<Self>, node: Rc<dyn Node>);
}

impl dyn ViewDesc {
    pub fn pos_before_child(self: Rc<Self>, child: Rc<dyn ViewDesc>) -> Position {
        let mut pos = self.clone().pos_at_start();

        for ch in self.children().iter() {
            if Rc::ptr_eq(ch, &child) {
                return pos;
            }

            pos += ch.size()
        }

        pos
    }

    pub fn pos_at_start(self: Rc<Self>) -> Position {
        if let Some(parent) = self.parent() {
            parent.pos_before_child(self)
        } else {
            0
        }
    }

    pub fn matches_node(&self, node: Rc<dyn Node>) -> bool {
        Rc::ptr_eq(&self.node(), &node)
    }

    pub fn mount_children(&self) {
        if let Some(parent) = self.content_dom() {
            for child in self.children().iter() {
                parent.append_child(&child.dom());
                child.mount_children();
            }
        }
    }
}