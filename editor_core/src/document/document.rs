use std::rc::Rc;
use crate::node::node::Node;
use crate::slice::slice::Slice;
use crate::document::replace::replace;
use crate::position::resolved_position::ResolvedPosition;

struct Document {
    root: Rc<Node>,
}

impl Document {
    pub fn root(&self) -> &Rc<Node> {
        &self.root
    }
    pub fn replace(&self, from: usize, to: usize, slice: Slice) -> Result<Self, String> {
        Ok(Self {
            root: replace(self.resolve(from)?, self.resolve(to)?, slice)?,
        })
    }
    pub fn resolve(&self, position: usize) -> Result<ResolvedPosition, String> {
        ResolvedPosition::resolve(&self.root, position)
    }
}

#[cfg(test)]
mod test {}
