// use std::rc::Rc;
// use crate::slice::slice::Slice;
// use crate::node::Node;
// use crate::node::resolved_position::ResolvedPosition;
//
// pub fn replace(
//     base: Rc<dyn Node>,
//     from: usize,
//     to: usize,
//     slice: Slice
// ) -> Result<Rc<dyn Node>, String> {
//     let resolved_from = base.resolve_offset(from)?;
//     let resolved_to = base.resolve_offset(to)?;
//
//     if slice.open_start() > resolved_from.depth() {
//         Err(format!("Inserted content deeper than insertion position."))
//     } else if slice.open_start() - resolved_from.depth() != slice.open_end() - resolved_to.depth() {
//         Err(format!("Inconsistent open depths."))
//     } else {
//         replace_outer(resolved_from, resolved_to, slice, 0)
//     }
// }
//
// pub fn replace_outer(
//     from: Rc<ResolvedPosition>,
//     to: Rc<ResolvedPosition>,
//     slice: Slice,
//     depth: usize,
// ) -> Result<Rc<dyn Node>, String> {
//     let from_index = from.index(depth)?;
//     let to_index = to.index(depth)?;
//     let node =
// }