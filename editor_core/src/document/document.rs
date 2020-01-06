use std::rc::Rc;
use crate::node::node::Node;
use crate::slice::slice::Slice;
use crate::document::replace::replace;
use crate::position::resolved_position::ResolvedPosition;

struct Document {
    root: Rc<Node>,
}

impl Document {
    pub fn replace(&self, from: usize, to: usize, slice: Slice) -> Result<Rc<Node>, String> {
        replace(self.resolve(from)?, self.resolve(to)?, slice)
    }
    pub fn resolve(&self, position: usize) -> Result<ResolvedPosition, String> {
        ResolvedPosition::resolve(&self.root, position)
    }
}
