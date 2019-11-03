use crate::view::virtual_node::VirtualNode;
use std::rc::Rc;

pub enum Dirty {
    NotDirty,
    ChildrenDirty,
    ContentDirty,
    NodeDirty,
}

pub struct ViewDescription {
    parent: Rc<ViewDescription>,
    children: Vec<Rc<ViewDescription>>,
    render_object: Box<VirtualNode>,
    dirty: Dirty,
}

impl ViewDescription {
    fn create(
        parent: Rc<ViewDescription>,
        children: Vec<Rc<ViewDescription>>,
        render_object: Box<VirtualNode>,
    ) -> Self {
        Self {
            parent,
            children,
            render_object,
            dirty: Dirty::NotDirty,
        }
    }
}
