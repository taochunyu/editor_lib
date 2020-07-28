use std::rc::Rc;
use std::cell::{RefCell, Ref};
use renderer::html::node::HTMLNode;
use renderer::Renderer;
use renderer::html::element::HTMLElement;
use renderer::html::text::HTMLTextNode;
use std::any::Any;
use crate::view::view_desc::ViewDesc;
use crate::node::Node;

struct TextViewDescMeta {
    parent: Option<Rc<dyn ViewDesc>>,
    node: Rc<dyn Node>,
    dom: HTMLTextNode,
}

pub struct TextViewDesc {
    meta: RefCell<TextViewDescMeta>,
    renderer: Rc<Renderer>,
}

impl ViewDesc for TextViewDesc {
    fn parent(&self) -> Option<Rc<dyn ViewDesc>> {
        self.meta.borrow().parent.clone()
    }

    fn children(&self) -> Option<Ref<Vec<Rc<dyn ViewDesc>>>> {
        None
    }

    fn node(&self) -> Rc<dyn Node> {
        self.meta.borrow().node.clone()
    }

    fn dom(&self) -> HTMLNode {
        self.meta.borrow().dom.clone().into()
    }

    fn content_dom(&self) -> Option<HTMLElement> {
        None
    }

    fn size(&self) -> usize {
        self.meta.borrow().node.size()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(self: Rc<Self>, node: Rc<dyn Node>) -> bool {
        let meta = self.meta.borrow_mut();

        HTMLNode::from(meta.dom.clone()).log(format!("here2"));

        if !meta.node.clone().value_eq(node.clone()) {
            return false;
        }
        HTMLNode::from(meta.dom.clone()).log(format!("here1"));

        if let Some(text) = node.as_text() {
            HTMLNode::from(meta.dom.clone()).log(format!("here"));
            meta.dom.set_node_value(Some(text.content().as_str()))
        }

        true
    }

    fn destroy(&self) {}

    fn to_debug_string(&self) -> String {
        format!("{}", self.node().serialize())
    }
}

impl TextViewDesc {
    pub fn new(
        parent: Option<Rc<dyn ViewDesc>>,
        node: Rc<dyn Node>,
        dom: HTMLTextNode,
        renderer: Rc<Renderer>,
    ) -> Rc<dyn ViewDesc> {
        Rc::new(Self {
            meta: RefCell::new(TextViewDescMeta {
                parent,
                node,
                dom,
            }),
            renderer,
        })
    }
}