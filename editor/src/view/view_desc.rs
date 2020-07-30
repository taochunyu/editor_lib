use std::rc::Rc;
use std::cell::{RefMut, RefCell, Ref};
use std::any::Any;
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::node::Node;
use crate::Position;
use crate::view::node_view_desc::NodeViewDesc;
use crate::view::text_view_desc::TextViewDesc;

pub trait ViewDesc {
    fn parent(&self) -> Option<Rc<dyn ViewDesc>>;
    fn children(&self) -> Option<Ref<Vec<Rc<dyn ViewDesc>>>>;
    fn node(&self) -> Rc<dyn Node>;
    fn dom(&self) -> HTMLNode;
    fn content_dom(&self) -> Option<HTMLElement>;
    fn size(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn update(self: Rc<Self>, node: Rc<dyn Node>) -> bool;
    fn destroy(&self);
    fn to_debug_string(&self) -> String;
    fn debug_log(&self, tag: &str, info: String);
}

impl dyn ViewDesc {
    pub fn pos_before_child(self: Rc<Self>, child: Rc<dyn ViewDesc>) -> Position {
        let mut pos = self.clone().pos_at_start();

        if let Some(children) = self.children() {
            for ch in children.iter() {
                if Rc::ptr_eq(ch, &child) {
                    return pos;
                }

                pos += ch.size()
            }
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
        // self.debug_log("matches", format!("{} match {}", self.node().serialize(), node.serialize()));
        self.node().value_eq(node)
    }

    pub fn mount_children(&self) {
        // self.debug_log("will mount dom", self.to_debug_string());
        // self.debug_log("will mount dom", String::new());

        if let Some(parent_dom) = self.content_dom() {
            if let Some(children) = self.children() {
                let mut dom = parent_dom.first_child();

                for child in children.iter() {
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
}

pub fn create_node_or_text_view_desc(
    parent: Option<Rc<dyn ViewDesc>>,
    node: Rc<dyn Node>,
    pos: Position,
    renderer: Rc<Renderer>,
) -> Rc<dyn ViewDesc> {
    if let Some(text) = node.clone().as_text() {
        let dom = text.render_text(renderer.clone());

        TextViewDesc::new(parent, node, dom, renderer)
    } else {
        let (dom, content_dom) = node.clone().render(renderer.clone());

        NodeViewDesc::new(parent, node, dom, content_dom, pos, renderer)
    }
}

fn remove_dom(dom: HTMLNode) -> Option<HTMLNode> {
    let next = dom.next_sibling();

    dom.remove();

    next
}
