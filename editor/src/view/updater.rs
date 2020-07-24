use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use renderer::Renderer;
use crate::node::Node;
use crate::view::View;
use crate::Position;
use crate::node::fragment::Fragment;
use crate::view::view_desc::ViewDesc;
use crate::view::node_view_desc::NodeViewDesc;

pub struct Updater<'a> {
    view_desc: Rc<dyn ViewDesc>,
    node: Rc<dyn Node>,
    children: &'a RefCell<Vec<Rc<dyn ViewDesc>>>,
    index: usize,
    changed: bool,
    pre_matched: Vec<Rc<dyn ViewDesc>>,
    pre_match_offset: usize,
    renderer: Rc<Renderer>,
}

struct PreMatchResult {
    nodes: Vec<Rc<dyn ViewDesc>>,
    offset: usize,
}

impl<'a> Updater<'a> {
    pub fn new(
        top: Rc<dyn ViewDesc>,
        children: &'a RefCell<Vec<Rc<dyn ViewDesc>>>,
        node: Rc<dyn Node>,
        renderer: Rc<Renderer>,
    ) -> Updater {
        let pre = Self::pre_match(node.clone().children(), children.borrow());

        Self { view_desc: top, children, node, index: 0, changed: false, renderer, pre_matched: pre.nodes, pre_match_offset: pre.offset }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn find_node_match(&mut self, node: Rc<dyn Node>, index: usize) -> bool {
        let mut found: Option<usize> = None;

        if let Some(pre_match) = self.get_pre_match(index) {
            if pre_match.matches_node(node.clone()) {
                found = self.view_desc.children().iter().position(|p| Rc::ptr_eq(p, &pre_match))
            }
        }

        if found.is_none() {
            let children_length = self.view_desc.children().len();
            let mut i = self.index;
            let mut end = if children_length > i + 5 { i + 5 } else { children_length };

            while i < end {
                if let Some(child) = self.view_desc.children().get(i) {
                    let is_matched = child.matches_node(node.clone());
                    let is_in_pre_match = self.view_desc.children().iter().position(|p| Rc::ptr_eq(p, child));

                    if  is_matched && is_in_pre_match.is_none() {
                        found = Some(i);

                        break;
                    }
                }

                i += 1;
                end = if children_length > i + 5 { i + 5 } else { children_length }
            }
        }

        if let Some(found) = found {
            self.destroy_between(self.index, found);
            self.index += 1;

            true
        } else {
            false
        }
    }

    pub fn update_next_node(&mut self) -> bool { false }

    pub fn add_node(&mut self, node: Rc<dyn Node>, pos: Position) {
        let node_view = NodeViewDesc::create(Some(self.view_desc.clone()), node, pos, self.renderer.clone());

        self.children.borrow_mut().insert(self.index, node_view);
        self.index += 1;
        self.changed = true;
    }

    pub fn destroy_between(&self, from: usize, to: usize) {}

    fn get_pre_match(&self, index: usize) -> Option<Rc<dyn ViewDesc>> {
        if index >= self.pre_match_offset {
            self.pre_matched.get(index - self.pre_match_offset).map(|v| v.clone())
        } else {
            None
        }
    }

    fn pre_match(fragment: Option<Rc<Fragment>>, descs: Ref<Vec<Rc<dyn ViewDesc>>>) -> PreMatchResult {
        let mut result: Vec<Rc<dyn ViewDesc>> = vec![];
        let mut end: usize = 0;

        if let Some(fragment) = fragment {
            for desc in descs.iter().rev() {
                if end == 0 {
                    break;
                }

                if let Ok(node) = fragment.get(end - 1) {
                    if !Rc::ptr_eq(&node, &desc.clone().node()) {
                        break;
                    }
                }

                result.push(desc.clone());
                end += 1;
            }
        }

        result.reverse();

        PreMatchResult { nodes: result, offset: end }
    }
}
