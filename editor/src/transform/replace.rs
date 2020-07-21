use std::rc::Rc;
use crate::node::slice::Slice;
use crate::transform::step::{Step, StepResult};
use crate::node::Node;
use crate::Doc;
use crate::transform::Transform;
use crate::transform::step_map::StepMap;
use crate::transform::mapping::Mapping;

pub struct ReplaceStep {
    from: usize,
    to: usize,
    slice: Slice,
    structure: bool,
}

impl Step for ReplaceStep {
    fn apply(&self, doc: Doc) -> StepResult {
        let is_content_between = match Self::content_between(doc.clone(), self.from, self.to) {
            Ok(is_content_between) => is_content_between,
            Err(err) => return StepResult::failed(err),
        };

        if self.structure && is_content_between {
            StepResult::failed(format!("Structure replace would overwrite content."))
        } else {
            StepResult::from_replace(doc, self.from, self.to, self.slice.clone())
        }
    }

    fn invert(&self, doc: Doc) -> Result<Box<dyn Step>, String> {
        let slice = doc.slice(self.from, self.to)?;

        Ok(Self::new(self.from, self.from + self.slice.size(), slice, false))
    }

    fn get_map(&self) -> StepMap {
        StepMap::new(vec![(self.from, self.to - self.from, self.slice.size())], false)
    }

    fn map(&self, mapping: Mapping) -> Option<Box<dyn Step>> {
        unimplemented!()
    }
}

impl ReplaceStep {
    pub fn new(from: usize, to: usize, slice: Slice, structure: bool) -> Box<dyn Step> {
        Box::new(ReplaceStep { from, to, slice, structure })
    }

    fn content_between(doc: Doc, from: usize, to: usize) -> Result<bool, String> {
        if to < from {
            Err(format!("From {} should not be greater than to {}.", from, to))
        } else {
            let from = doc.resolve(from)?;
            let mut depth = from.depth();
            let mut dist = to - from.offset();

            while depth > 0 && dist > 0 && from.index_after(depth)? == from.step(depth)?.node.child_count() {
                depth -= 1;
                dist -= 1;
            }

            if dist > 0 {
                let mut next = from.step(depth)?.node.get_child(from.index_after(depth)?);

                while dist > 0 {
                    if let Ok(node) = &next {
                        if node.children().is_some() {
                            return Ok(true);
                        }

                        next = node.get_child(0);
                        dist -= 1;
                    }

                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}

impl Transform {
    pub fn replace(&mut self, from: usize, to: usize, slice: Slice) -> &mut Self {
        let step = ReplaceStep::new(from, to, slice, false);

        self.step(step);

        self
    }
}
