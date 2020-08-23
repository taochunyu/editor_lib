use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use renderer::Renderer;
use crate::node::Node;
use crate::view::view_desc::ViewDesc;
use crate::Position;
use crate::view::updater::Updater;
use std::any::Any;
use renderer::host::ExtraInfo;
use crate::view::TypedExtraInfo;

struct NodeViewDescMeta {
    parent: Option<Rc<dyn ViewDesc>>,
    node: Rc<dyn Node>,
    dom: HTMLNode,
    content_dom: Option<HTMLElement>,
}

pub struct NodeViewDesc {
    meta: RefCell<NodeViewDescMeta>,
    children: RefCell<Vec<Rc<dyn ViewDesc>>>,
    renderer: Rc<Renderer>,
}

impl ViewDesc for NodeViewDesc {
    fn parent(&self) -> Option<Rc<dyn ViewDesc>> {
        self.meta.borrow().parent.clone()
    }

    fn children(&self) -> Option<Ref<Vec<Rc<dyn ViewDesc>>>> {
        Some(self.children.borrow())
    }

    fn node(&self) -> Rc<dyn Node> {
        self.meta.borrow().node.clone()
    }

    fn dom(&self) -> HTMLNode {
        self.meta.borrow().dom.clone()
    }

    fn content_dom(&self) -> Option<HTMLElement> {
        self.meta.borrow().content_dom.clone()
    }

    fn border(&self) -> usize {
        if self.meta.borrow().node.children().is_none() { 0 } else { 1 }
    }

    fn size(&self) -> usize {
        self.meta.borrow().node.size()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(self: Rc<Self>, node: Rc<dyn Node>) -> bool {
        if !self.node().same_mark_up(node.clone()) {
            false
        } else {
            self.update_inner(node);

            true
        }
    }

    fn destroy(&self) {
        if let Some(children) = self.children() {
            for child in children.iter() {
                child.destroy();
            }
        }
    }

    fn to_debug_string(&self) -> String {
        let content = self.meta.borrow().node.serialize();
        let children = self.children.borrow().iter()
            .map(|child| child.to_debug_string())
            .collect::<Vec<String>>()
            .join("\n");

        format!("({})[{}]", content, children)
    }

    fn debug_log(&self, tag: &str, info: String) {
        self.renderer.log(tag, info);
    }
}

impl NodeViewDesc {
    pub fn new(
        parent: Option<Rc<dyn ViewDesc>>,
        node: Rc<dyn Node>,
        dom: HTMLNode,
        content_dom: Option<HTMLElement>,
        pos: Position,
        renderer: Rc<Renderer>,
    ) -> Rc<dyn ViewDesc> {
        let node_view_desc = Rc::new(Self {
            meta: RefCell::new(NodeViewDescMeta {
                parent,
                node: node.clone(),
                dom: dom.clone(),
                content_dom,
            }),
            children: RefCell::new(vec![]),
            renderer,
        });

        dom.set_extra_info(TypedExtraInfo::new(node_view_desc.clone()));

        if node_view_desc.clone().update_children(node, pos) {
            (node_view_desc.clone() as Rc<dyn ViewDesc>).mount_children();
        };

        node_view_desc
    }

    pub fn create(
        parent: Option<Rc<dyn ViewDesc>>,
        node: Rc<dyn Node>,
        pos: Position,
        renderer: Rc<Renderer>,
    ) -> Rc<dyn ViewDesc> {
        let (dom, content_dom) = node.clone().render(renderer.clone());

        Self::new(parent, node, dom, content_dom, pos, renderer)
    }

    fn update_inner(self: Rc<Self>, node: Rc<dyn Node>) {
        let need_update_children = {
            let mut meta = self.meta.borrow_mut();

            meta.node = node.clone();

            meta.content_dom.is_some()
        };

        if need_update_children {
            let self_trait = self.clone() as Rc<dyn ViewDesc>;
            let pos = self_trait.clone().pos_at_start();
            let updated = self.update_children(node, pos);

            if updated {
                self_trait.mount_children();
            }
        }

    }

    fn update_children(self: Rc<Self>, node: Rc<dyn Node>, pos: Position) -> bool {
        let mut updater = Updater::new(self.clone(), &self.children, node.clone(), self.renderer.clone());

        self.debug_log("update node desc children", format!("length {}: {}", node.child_count(), node.clone().serialize()));

        if let Some(children) = node.clone().children() {
            for (index, child) in children.content().iter().enumerate() {
                self.debug_log("update a child node", child.clone().serialize());

                if updater.find_node_match(child.clone(), index) {
                    self.debug_log("found node match", child.clone().serialize());
                    continue;
                }

                if updater.update_next_node(child.clone(), index) {
                    self.debug_log("updated node desc", child.clone().serialize());
                    continue;
                }

                updater.add_node(child.clone(), pos);

                self.debug_log("added a new node", child.clone().serialize());
            }
        }

        updater.destroy_rest();

        updater.changed()
    }
}
