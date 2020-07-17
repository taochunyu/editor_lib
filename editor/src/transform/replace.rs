use std::rc::Rc;
use crate::node::slice::Slice;
use crate::transform::step::{Step, StepResult};
use crate::node::Node;
use crate::transform::step_map::Mapping;
use crate::Doc;

struct ReplaceStep {
    from: usize,
    to: usize,
    slice: Rc<Slice>,
    structure: bool,
}

impl Step for ReplaceStep {
    fn apply(&self, doc: Rc<Doc>) -> StepResult {
        if self.structure {
            StepResult::err(format!("Structure replace would overwrite content."))
        } else {
            StepResult::from_replace(doc, self.from, self.to, self.slice.clone())
        }
    }

    fn invert(&self, doc: Rc<Doc>) -> Result<Box<dyn Step>, String> {
        let slice = Rc::new(doc.slice(self.from, self.to))?;

        Ok(Self::new(self.from, self.from + self.slice.size(), slice, false))
    }

    fn map(&self, mapping: Mapping) -> Option<Box<dyn Step>> {
        unimplemented!()
    }
}

impl ReplaceStep {
    fn new(from: usize, to: usize, slice: Rc<Slice>, structure: bool) -> Box<dyn Step> {
        Box::new(ReplaceStep { from, to, slice, structure })
    }

    fn content_between(doc: Rc<Doc>, from: usize, to: usize) -> Result<bool, String> {
        if to < from {
            Err(format!("From {} should not be greater than to {}.", from, to))
        } else {
            let from = doc.resolve(from)?;
            let mut depth = from.depth();
            let mut dist = to - from;

            while depth > 0 && dist > 0 && from.index_after(depth) == from.step(depth)?.node.child_count() {
                depth -= 1;
                dist -= 1;
            }

            if dist > 0 {
                let mut next = from.step(depth)?.node.get_child(from.index_after(depth)?)?;

                while dist > 0 {

                    dist -= 1;
                }
            } else {
                Ok(false)
            }
        }
    }
}