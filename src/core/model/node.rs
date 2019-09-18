use crate::core::model::fragment::Fragment;

pub trait Node {
    fn is_text(&self) -> bool { false }
    fn is_leaf(&self) -> bool { false }
    fn border_size(&self) -> usize {
        if self.is_text() { 0 }
        else if self.is_leaf() { 1 } else { 2 }
    }
    fn to_string(&self) -> String {
        String::from("")
    }
}

pub struct TreeNode {
    pub(crate) content: Box<Fragment>,
    node: Box<dyn Node>,
}

impl TreeNode {
    pub(crate) fn content(&self) -> &Fragment {
        &self.content
    }
    pub(crate) fn size(&self) -> usize {
        self.node.border_size() + self.content.size()
    }
    fn to_string(&self) -> String {
        self.node.to_string()
    }
    fn is_text(&self) -> bool {
        self.node.is_text()
    }
    fn has_same_attrs(&self, other: &TreeNode) -> bool { true }
    pub(crate) fn copy(self, content: Box<Fragment>) -> Box<TreeNode> {
        Box::new(TreeNode {
            node: self.node,
            content,
        })
    }
}
