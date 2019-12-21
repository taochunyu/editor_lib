use std::rc::Rc;
use crate::node::node::Node;
use crate::position::resolved_position::ResolvedPosition;
use crate::slice::slice::Slice;

pub fn replace(
    from: ResolvedPosition,
    to: ResolvedPosition,
    slice: Slice
) -> Result<Rc<Node>, String> {
    if slice.open_start() > from.depth() {
        Err(format!("Inserted content deeper than insertion position"))
    } else if slice.open_start() - from.depth() != slice.open_end() - to.depth() {
        Err(format!("Inconsistent open depths"))
    } else {
        replace_outer(from, to, slice, 0)
    }
}

fn replace_outer(
    from: ResolvedPosition,
    to: ResolvedPosition,
    slice: Slice,
    depth: usize
) -> Result<Rc<Node>, String> {

}
