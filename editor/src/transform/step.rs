use std::rc::Rc;
use crate::Doc;
use crate::transform::step_map::Mapping;
use crate::node::slice::Slice;

pub trait Step {
    fn apply(&self, doc: Rc<Doc>) -> StepResult;
    fn invert(&self, doc: Rc<Doc>) -> Box<dyn Step>;
    fn map(&self, mapping: Mapping) -> Option<Box<dyn Step>>;
}

pub struct StepResult {
    doc: Option<Rc<Doc>>,
    err: Option<String>,
}

impl StepResult {
    pub fn ok(doc: Rc<Doc>) -> Self {
        Self { doc: Some(doc), err: None }
    }

    pub fn err(err: String) -> Self {
        Self { doc: None, err: Some(err) }
    }

    pub fn from_replace(doc: Rc<Doc>, from: usize, to: usize, slice: Rc<Slice>) -> Self {
        match doc.replace(from, to, slice) {
            Ok(doc) => Self::ok(doc),
            Err(err) => Self::err(err),
        }
    }
}
