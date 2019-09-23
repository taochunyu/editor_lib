use crate::core::model::node::TreeNode;
use std::rc::Rc;

pub struct Fragment {
    pub(crate) content: Vec<Rc<TreeNode>>,
    pub(crate) size: usize,
}

impl Fragment {
    pub(crate) fn new(content: Vec<Rc<TreeNode>>, size: usize) -> Self {
        Self { content, size }
    }
    pub(crate) fn replace_child(&self, index: usize, tree_node: Rc<TreeNode>) -> Rc<Self> {
        let size = self.size + tree_node.size() - self.content[index].size();
        let mut content = vec![];

        for (i, node) in self.content.iter().enumerate() {
            if i != index {
                content.push(Rc::clone(node));
            } else {
                content.push(Rc::clone(&tree_node));
            }
        }

        Rc::new(Self { content, size })
    }
    pub(crate) fn append(this: &Rc<Self>, other: &Rc<Self>) -> Rc<Self> {
        if let Some(last_child) = this.content.last() {
            if let Some(first_child) = other.content.first() {
                let mut content = vec![];
                let cursor = if last_child.need_join(&first_child) {
                    1
                } else {
                    0
                };

                for (index, child) in this.content.iter().enumerate() {
                    if index + cursor < this.content.len() {
                        content.push(Rc::clone(child))
                    }
                }
                if cursor == 1 {
                    content.push(last_child.join(&first_child));
                }
                for (index, child) in other.content.iter().enumerate() {
                    if index + 1 > cursor {
                        content.push(Rc::clone(child))
                    }
                }
                Rc::new(Self::new(content, this.size + other.size))
            } else {
                Rc::clone(&this)
            }
        } else {
            Rc::clone(&other)
        }
    }
    pub(crate) fn cut(&self, from: usize, to: usize) -> Rc<Self> {
        let mut content: Vec<Rc<TreeNode>> = vec![];
        let mut size = 0;
        let mut pos = 0;

        if to > from {
            for (index, tree_node) in self.content.iter().enumerate() {
                if pos >= to {
                    break;
                }

                let end = pos + tree_node.size();

                if end > from {
                    let child = if pos < from || end > to {
                        let deep = if tree_node.is_text() {
                            let cut_from = if from > pos { from - pos } else { 0 };
                            let cut_to = if tree_node.size() + pos < to {
                                tree_node.size()
                            } else {
                                to - pos
                            };
                            tree_node.cut(cut_from, cut_to)
                        } else {
                            let cut_from = if from > pos + 1 { from - pos - 1 } else { 0 };
                            let tree_node_content_size = match &tree_node.content {
                                Some(content) => content.size,
                                None => 0,
                            };
                            let cut_to = if tree_node_content_size + 1 + pos < to {
                                tree_node_content_size
                            } else {
                                to - pos - 1
                            };
                            tree_node.cut(cut_from, cut_to)
                        };
                        Rc::clone(&deep)
                    } else {
                        Rc::clone(tree_node)
                    };
                    size += child.size();
                    content.push(child);
                }
                pos = end;
            }
        }

        Rc::new(Self::new(content, size))
    }
}
