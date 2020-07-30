use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};
use renderer::Renderer;
use crate::node::Node;
use crate::view::View;
use crate::Position;
use crate::node::fragment::Fragment;
use crate::view::view_desc::{ViewDesc, create_node_or_text_view_desc};
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
            let children = self.children.borrow();

            if pre_match.matches_node(node.clone()) {
                found = children.iter().position(|p| Rc::ptr_eq(p, &pre_match))
            }
        }

        if found.is_none() {
            let children = self.children.borrow();
            let children_length = children.len();
            let mut i = self.index;
            let end = if children_length > i + 5 { i + 5 } else { children_length };

            while i < end {
                if let Some(child) = children.get(i) {
                    let is_matched = child.matches_node(node.clone());
                    let is_in_pre_match = children.iter().position(|p| Rc::ptr_eq(p, child));

                    if  is_matched && is_in_pre_match.is_none() {
                        found = Some(i);

                        break;
                    }
                }

                i += 1;
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

    pub fn update_next_node(&mut self, node: Rc<dyn Node>, index: usize) -> bool {
        let mut found: Option<usize> = None;

        for i in self.index..self.children.borrow().len() {
            if let Some(next) = self.children.borrow().get(i) {
                let pre_match = self.pre_matched.iter().position(|desc| Rc::ptr_eq(desc, &next.clone()));

                if let Some(pre_match) = pre_match {
                    if pre_match + self.pre_match_offset != index {
                        return false;
                    }
                }

                if next.clone().update(node) {
                    found = Some(i);
                }

                break;
            }
        }

        if let Some(found) = found {
            self.destroy_between(self.index, found);
            self.changed = true;
            self.index += 1;

            true
        } else {
            false
        }
    }

    pub fn add_node(&mut self, node: Rc<dyn Node>, pos: Position) {
        let node_view = create_node_or_text_view_desc(Some(self.view_desc.clone()), node, pos, self.renderer.clone());

        self.children.borrow_mut().insert(self.index, node_view);
        self.index += 1;
        self.changed = true;
    }

    pub fn destroy_between(&mut self, from: usize, to: usize) {
        for child in self.children.borrow_mut().drain(from..to) {
            child.destroy();
        }

        self.changed = true;
    }

    pub fn destroy_rest(&mut self) {
        let to = { self.children.borrow().len() };

        self.destroy_between(self.index, to);
    }

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
            end = fragment.count();

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
                end -= 1;
            }
        }

        result.reverse();

        PreMatchResult { nodes: result, offset: end }
    }
}
