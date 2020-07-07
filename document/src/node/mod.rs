use std::any::Any;
use std::rc::Rc;
use std::ops::Range;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::element::Element;
use crate::node::text::Text;
use crate::node::path::Path;
use crate::node::fragment::Fragment;
use crate::node::slice::Slice;
use crate::node::replace::replace;
use crate::view::View;
use std::cell::RefCell;

pub mod element_type;
mod fragment;
pub mod slice;
pub mod element;
pub mod text;
mod path;
mod replace;

pub trait Node {
    fn size(&self) -> usize;
    fn content_size(&self) -> usize;
    fn child_count(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String>;
    fn index(&self, offset: usize) -> Result<usize, String>;
    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String>;
    fn children(&self) -> Option<Rc<Fragment>>;
    fn replace_children(&self, new_children: Option<Rc<Fragment>>) -> Result<Rc<dyn Node>, String>;
    fn to_html_string(&self) -> String;
    fn render(self: Rc<Self>, view: Rc<View>) -> (OuterDOM, ContentDOM);
}

impl dyn Node {
    fn as_text(&self) -> Option<&Text> {
        self.as_any().downcast_ref::<Text>()
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
            Ok(self.cut_node(from, to)?)
        }
    }

    pub fn find_path(self: Rc<Self>, offset: usize) -> Result<Rc<Path>, String> {
        Path::new(self.clone(), offset)
    }

    pub fn replace(self: Rc<Self>, from: usize, to: usize, slice: Slice) -> Result<Rc<dyn Node>, String> {
        replace(self, from, to, slice)
    }
}

pub fn create_node<T: ElementType>(
    attrs: T::Attributes,
    children: Option<Vec<Rc<dyn Node>>>,
) -> Rc<dyn Node> {
    T::create(Rc::new(attrs), children)
}

pub fn create_text_node(content: &str) -> Rc<dyn Node> {
    Text::new(String::from(content))
}