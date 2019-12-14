use std::rc::Rc;
use crate::node::node::Node;
use crate::position::resolved_position::ResolvedPosition;
use crate::slice::slice::Slice;

pub fn replace(
    root: &Rc<Node>,
    from: ResolvedPosition,
    to: ResolvedPosition,
    slice: Slice
) -> Result<Rc<Node>, String> {
    if slice.open_start() > from.depth() {

    }
}
