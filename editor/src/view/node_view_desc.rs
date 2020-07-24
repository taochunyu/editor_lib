use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use renderer::Renderer;
use crate::node::Node;
use crate::view::view_desc::ViewDesc;
use crate::Position;
use crate::view::updater::Updater;

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

    fn children(&self) -> Ref<Vec<Rc<dyn ViewDesc>>> {
        self.children.borrow()
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

    fn size(&self) -> usize {
        self.meta.borrow().node.size()
    }

    fn update(self: Rc<Self>, node: Rc<dyn Node>) {
        let self_ref = self.clone();
        let self_trait_object = self.clone() as Rc<dyn ViewDesc>;
        let pos = self_trait_object.pos_at_start();
        let mut meta = self.meta.borrow_mut();

        meta.node = node.clone();

        if meta.content_dom.is_some() {
            self_ref.update_children(node, pos);
        }
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
                dom,
                content_dom,
            }),
            children: RefCell::new(vec![]),
            renderer,
        });

        node_view_desc.clone().update_children(node, pos);

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

    fn update_children(self: Rc<Self>, node: Rc<dyn Node>, pos: Position) {
        let mut updater = Updater::new(self.clone(), &self.children, node.clone(), self.renderer.clone());

        if let Some(children) = node.clone().children() {
            for (index, child) in children.content().iter().enumerate() {
                if updater.find_node_match(child.clone(), index) {
                    break;
                }

                updater.add_node(child.clone(), pos)
            }
        }

        if !updater.changed() {
            return;
        }

        (self.clone() as Rc<dyn ViewDesc>).mount_children();
    }
}
