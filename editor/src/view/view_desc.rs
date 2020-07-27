use std::rc::Rc;
use std::cell::{RefMut, RefCell, Ref};
use std::any::Any;
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
    fn as_any(&self) -> &dyn Any;
    fn update(self: Rc<Self>, node: Rc<dyn Node>) -> bool;
    fn destroy(&self);
    fn to_debug_string(&self) -> String;
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
        self.node().eq(node)
    }

    pub fn mount_children(&self) {
        if let Some(parent_dom) = self.content_dom() {
            let mut dom = parent_dom.first_child();

            for child in self.children().iter() {
                let child_dom = child.dom();

                match child_dom.parent() {
                    Some(child_dom_parent) if child_dom_parent == parent_dom.clone().into() => {
                        while let Some(next) = dom.clone() {
                            if child_dom.eq(&next) {
                                break;
                            }

                            dom = remove_dom(next);
                        }

                        if let Some(next) = dom.clone() {
                            dom = next.next_sibling();
                        }
                    },
                    _ => {
                        parent_dom.insert_before(child_dom, dom.clone());
                    }
                }
            }

            while let Some(next) = dom.clone() {
                dom = remove_dom(next)
            }
        }
    }
}

fn remove_dom(dom: HTMLNode) -> Option<HTMLNode> {
    let next = dom.next_sibling();

    dom.remove();

    next
}
