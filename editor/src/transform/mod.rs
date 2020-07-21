pub mod mapping;
mod step_map;
pub mod mappable;
mod step;
mod replace;

use std::rc::Rc;
use crate::node::Node;
use crate::Doc;
use crate::node::slice::Slice;
use crate::transform::replace::ReplaceStep;
use crate::transform::step::{Step, StepResult};
use crate::transform::mapping::Mapping;

pub struct Transform {
    doc: Doc,
    docs: Vec<Doc>,
    steps: Vec<Box<dyn Step>>,
    mapping: Mapping,
}

impl Transform {
    pub fn new(doc: Doc) -> Self {
        Transform {
            doc,
            docs: vec![],
            steps: vec![],
            mapping: Mapping::new(vec![], None, 0, 0),
        }
    }

    pub fn doc(&self) -> Doc {
        self.doc.clone()
    }

    fn step(&mut self, step: Box<dyn Step>) -> Result<StepResult, String> {
        let step_result = step.apply(self.doc.clone());

        match &step_result {
            StepResult::Success(doc) => {
                self.add_step(step, doc.clone());

                Ok(step_result)
            },
            StepResult::Failed(reason) => Err(reason.clone()),
        }
    }

    fn add_step(&mut self, step: Box<dyn Step>, doc: Doc) {
        self.doc = doc.clone();
        self.docs.push(doc.clone());
        self.mapping.add_map(step.get_map(), None);
        self.steps.push(step);
    }
}

#[cfg(test)]
mod test {
    use crate::transform::Transform;
    use crate::test::tools::{create_doc, create_empty_slice};

    #[test]
    fn replace_works() {
        let doc = create_doc();
        let slice = create_empty_slice();
        let mut transform = Transform::new(doc);

        transform
            .replace(3, 4, slice.clone())
            .replace(3, 4, slice.clone());

        println!("{}", transform.doc.serialize());
    }
}