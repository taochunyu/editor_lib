pub mod utils;
pub mod element_type;
mod fragment;
pub mod slice;
pub mod element;
pub mod text;
mod path;
mod replace;

use std::any::Any;
use std::rc::Rc;
use std::ops::Range;
use crate::node::element_type::{OuterDOM, ContentDOM};
use crate::node::text::Text;
use crate::node::path::Path;
use crate::node::fragment::Fragment;
use crate::node::slice::Slice;
use crate::node::replace::replace;
use crate::view::View;

pub trait Node {
    fn type_name(&self) -> &str;
    fn size(&self) -> usize;
    fn content_size(&self) -> usize;
    fn child_count(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn cut(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String>;
    fn index(&self, offset: usize) -> Result<usize, String>;
    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String>;
    fn replace_child(&self, index: usize, child: Rc<dyn Node>) -> Result<Rc<dyn Node>, String> {
        unimplemented!()
    }
    fn children(&self) -> Option<Rc<Fragment>>;
    fn replace_children(&self, children: Rc<Fragment>) -> Result<Rc<dyn Node>, String>;
    fn serialize(&self) -> String;
    fn render(self: Rc<Self>, view: Rc<View>) -> (OuterDOM, ContentDOM);
}

impl dyn Node {
    fn as_text(&self) -> Option<&Text> {
        self.as_any().downcast_ref::<Text>()
    }

    fn is_text(&self) -> bool {
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

    fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        if from == 0 && to == self.content_size() {
            Ok(self.clone())
        } else {
            Ok(self.cut(from, to)?)
        }
    }

    pub fn find_path(self: Rc<Self>, offset: usize) -> Result<Rc<Path>, String> {
        Path::new(self.clone(), offset)
    }

    pub fn replace(self: Rc<Self>, from: usize, to: usize, slice: Slice) -> Result<Rc<dyn Node>, String> {
        replace(self, from, to, slice)
    }
}
