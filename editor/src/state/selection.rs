use std::rc::Rc;
use std::any::Any;
use crate::{Position, Doc};
use crate::node::path::Path;
use crate::transform::mapping::Mapping;
use crate::node::slice::Slice;
use crate::transform::mappable::{Mappable, AssociatedSide};
use crate::transform::Transform;

pub trait Selection {
    fn name() -> &'static str where Self: Sized;
    fn as_any(&self) -> &dyn Any;
    fn map(&self, doc: Doc, mapping: Mapping) -> Result<Rc<dyn Selection>, String>;
    fn content(&self) -> Result<Slice, String>;
    fn eq(&self, other: Rc<dyn Selection>) -> bool;
    fn replace(&self, tr: &mut Transform, slice: Slice);
}
