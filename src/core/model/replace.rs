use crate::core::model::node::TreeNode;
use crate::core::model::resolved_position::ResolvedPosition;
use crate::core::model::slice::Slice;

impl TreeNode {
    pub fn can_replace(
        &self,
        from: usize,
        to: usize,
        slice: Slice
    ) -> Result<(), String> {
        let resolved_from = self.resolve(from)?;
        let resolved_to = self.resolve(to)?;

        if slice.open_start > resolved_from.depth {
            return Err(String::from("Inserted content is deeper than insertion position"));
        }
        if resolved_from.depth - slice.open_start != resolved_to.depth - slice.open_end {
            return Err(String::from("Inconsistent open depths"));
        }
        Ok(())
    }

    fn replace(
        self,
        resolved_from: &ResolvedPosition,
        resolved_to: &ResolvedPosition,
        slice: Slice,
        depth: usize,
    ) -> Result<Box<TreeNode>, String> {
        let index = resolved_from.index(depth);

        if index == resolved_to.index(depth) && depth < resolved_from.depth - slice.open_start {
            let inner = self.content.content[index].replace(resolved_from, resolved_to, slice, depth + 1)?;
            let content = self.content.replace_child(index, inner);
            return Ok(self.copy(content));
        }

        Err(String::from(""))
    }
}
