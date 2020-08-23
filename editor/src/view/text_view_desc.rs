use std::rc::Rc;
use std::cell::{RefCell, Ref};
use renderer::html::node::HTMLNode;
use renderer::Renderer;
use renderer::html::element::HTMLElement;
use renderer::html::text::HTMLTextNode;
use std::any::Any;
use crate::view::view_desc::ViewDesc;
use crate::node::Node;
use crate::view::TypedExtraInfo;

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

    fn border(&self) -> usize {
        0
    }

    fn size(&self) -> usize {
        self.meta.borrow().node.size()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(self: Rc<Self>, node: Rc<dyn Node>) -> bool {
        let mut meta = self.meta.borrow_mut();

        if !meta.node.clone().same_mark_up(node.clone()) {
            return false;
        }

        if let Some(text) = node.clone().as_text() {
            meta.dom.set_node_value(Some(text.content().as_str()));
        }

        meta.node = node.clone();

        true
    }

    fn destroy(&self) {}

    fn to_debug_string(&self) -> String {
        format!("{}", self.node().serialize())
    }

    fn debug_log(&self, tag: &str, info: String) {
        self.renderer.log(tag, info);
    }
}

impl TextViewDesc {
    pub fn new(
        parent: Option<Rc<dyn ViewDesc>>,
        node: Rc<dyn Node>,
        dom: HTMLTextNode,
        renderer: Rc<Renderer>,
    ) -> Rc<dyn ViewDesc> {
        let desc = Rc::new(Self {
            meta: RefCell::new(TextViewDescMeta {
                parent,
                node,
                dom: dom.clone(),
            }),
            renderer,
        });

        HTMLNode::from(dom).set_extra_info(TypedExtraInfo::new(desc.clone()));

        desc
    }
}