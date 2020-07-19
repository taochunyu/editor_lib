use std::rc::Rc;
use crate::Doc;
use crate::transform::step_map::{Mapping, StepMap};
use crate::node::slice::Slice;

pub enum StepResult {
    Success(Doc),
    Failed(String),
}

impl StepResult {
    pub fn success(doc: Doc) -> Self {
        Self::Success(doc)
    }

    pub fn failed(reason: String) -> Self {
        Self::Failed(reason)
    }

    pub fn from_replace(doc: Doc, from: usize, to: usize, slice: Slice) -> Self {
        match doc.replace(from, to, slice) {
            Ok(doc) => Self::success(doc),
            Err(err) => Self::failed(err),
        }
    }
}

pub trait Step {
    fn apply(&self, doc: Doc) -> StepResult;
    fn invert(&self, doc: Doc) -> Result<Box<dyn Step>, String>;
    fn get_map(&self) -> StepMap;
    fn map(&self, mapping: Mapping) -> Option<Box<dyn Step>>;
}
