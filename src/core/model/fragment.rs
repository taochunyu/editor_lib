use crate::core::model::node::TreeNode;

pub struct Fragment {
    pub(crate) content: Vec<Box<TreeNode>>,
    size: usize,
}

impl Fragment {
    fn empty() -> Fragment {
        Fragment {
            content: vec![],
            size: 0,
        }
    }
    fn new(content: Vec<Box<TreeNode>>, size: usize) -> Fragment {
        Fragment {
            content,
            size,
        }
    }
    pub(crate) fn content(&self) -> &Vec<Box<TreeNode>> {
        &self.content
    }
    pub(crate) fn size(&self) -> usize {
        self.size
    }
    fn first_child(&self) -> Option<&Box<TreeNode>> {
        self.content.first()
    }
    fn last_child(&self) -> Option<&Box<TreeNode>> {
        self.content.last()
    }
    fn child(self, index: usize) -> Result<Box<TreeNode>, String> {
        if index < self.content.len() {
            Ok(self.content[index])
        } else {
            Err(format!("Index {} out of Range", index))
        }
    }
    pub(crate) fn replace_child(mut self, index: usize, tree_node: Box<TreeNode>) -> Box<Fragment> {
        let size = self.size + tree_node.size() - self.content[index].size();

        self.content[index] = tree_node;

        Box::new(Fragment {
            content: self.content,
            size,
        })
    }
}
