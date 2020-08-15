pub mod utils;
pub mod fragment;
pub mod slice;
pub mod element;
pub mod text;
pub mod path;
mod replace;

use std::any::Any;
use std::rc::Rc;
use std::ops::Range;
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::node::text::Text;
use crate::node::path::Path;
use crate::node::fragment::Fragment;
use crate::node::slice::Slice;
use crate::node::replace::replace;

pub trait Node {
    fn type_name(&self) -> &str;
    fn size(&self) -> usize;
    fn content_size(&self) -> usize;
    fn child_count(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<dyn Node>, String>;
    fn index(&self, offset: usize) -> Result<usize, String>;
    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String>;
    fn replace_child(&self, index: usize, child: Rc<dyn Node>) -> Result<Rc<dyn Node>, String>;
    fn children(&self) -> Option<Rc<Fragment>>;
    fn replace_children(&self, children: Rc<Fragment>) -> Result<Rc<dyn Node>, String>;
    fn serialize(&self) -> String;
    fn render(self: Rc<Self>, renderer: Rc<Renderer>) -> (HTMLNode, Option<HTMLElement>);
    fn same_mark_up(self: Rc<Self>, other: Rc<dyn Node>) -> bool;
    fn value_eq(self: Rc<Self>, other: Rc<dyn Node>) -> bool;
}

impl dyn Node {
    pub fn as_text(&self) -> Option<&Text> {
        self.as_any().downcast_ref::<Text>()
    }

    pub fn is_text(&self) -> bool {
        self.as_text().is_some()
    }

    fn join(&self, node: Rc<dyn Node>) -> Option<Rc<dyn Node>> {
        if let Some(a) = self.as_text() {
            if let Some(b) = node.as_text() {
                return a.try_concat(b)
            }
        }

        None
    }

    fn get_child_range(&self, range: Range<usize>) -> Result<Vec<Rc<dyn Node>>, String> {
        let mut collect: Vec<Rc<dyn Node>> = vec![];

        for index in range.step_by(1) {
            collect.push(self.get_child(index)?)
        }

        Ok(collect)
    }

    pub fn resolve(self: Rc<Self>, offset: usize) -> Result<Path, String> {
        Path::new(self.clone(), offset)
    }

    pub fn replace(self: Rc<Self>, from: usize, to: usize, slice: Slice) -> Result<Rc<dyn Node>, String> {
        replace(self, from, to, slice)
    }

    pub fn slice(self: Rc<Self>, from: usize, to: usize) -> Result<Slice, String> {
        if from > self.content_size() || to > self.content_size() {
            return Err(format!("Offset {} of {} outside of node.", from, to));
        }

        if from >= to {
            Ok(Slice::empty())
        } else {
            let from = self.clone().resolve(from)?;
            let to = self.clone().resolve(to)?;
            let depth = from.shared_depth(to.offset())?;
            let start = from.start(depth)?;
            let node = from.step(depth)?.node;
            let node = node.cut(from.offset() - start, to.offset() - start)?;

            if node.is_text() {
                Ok(Slice::from(node))
            } else {
                match node.children() {
                    Some(children) => Ok(Slice::from(children)),
                    None => Ok(Slice::empty()),
                }
            }
        }
    }
}
