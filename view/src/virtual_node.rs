use std::collections::HashMap;
use std::rc::Rc;

pub struct VirtualNode {
    pub key: String,
    pub tag: String,
    pub props: String,
    pub children: Vec<Rc<VirtualNode>>,
    count: usize,
}

pub enum Patch {
    Replace(Rc<VirtualNode>),
    Reorder,
    Props(String),
}

impl VirtualNode {
    pub fn create(key: &str, tag: &str, props: &str, children: Vec<Rc<VirtualNode>>) -> Self {
        let count = children.iter().fold(0, |acc, x| acc + x.count);

        Self {
            key: String::from(key),
            tag: String::from(tag),
            props: String::from(props),
            children,
            count,
        }
    }

    pub fn diff(
        old_node: Rc<VirtualNode>,
        new_node: Rc<VirtualNode>,
    ) -> HashMap<usize, Vec<Patch>> {
        let mut index: usize = 0;
        let mut patches: HashMap<usize, Vec<Patch>> = HashMap::new();

        dfs_walker(&old_node, Some(&new_node), index, &mut patches);

        patches
    }

    pub fn to_string_fn(&self, depth: usize) -> String {
        let mut prefix = String::from("");
        let mut children_string = String::from("");

        for _ in 0..depth + 1 {
            prefix.push_str("    ");
        }

        for child in &self.children {
            children_string.push_str(format!("\n{}{}", prefix.as_str(), child.to_string_fn(depth + 1)).as_str())
        }

        format!("{} {} {} {}", self.tag, self.key, self.props, children_string)
    }

    pub fn to_string(&self) -> String {
        self.to_string_fn(0)
    }
}

fn dfs_walker(
    old_node: &Rc<VirtualNode>,
    new_node_opt: Option<&Rc<VirtualNode>>,
    index: usize,
    patches: &mut HashMap<usize, Vec<Patch>>,
) {
    let mut current_patch: Vec<Patch> = vec![];

    if let Some(new_node) = new_node_opt {
        if old_node.tag == new_node.tag && old_node.key == new_node.key {
            if old_node.props != new_node.props {
                current_patch.push(Patch::Props(new_node.props.clone()));
            };

            diff_children(
                &old_node.children,
                &new_node.children,
                index,
                patches,
                &mut current_patch,
            );
        } else {
            current_patch.push(Patch::Replace(Rc::clone(new_node)));
        };
    };

    patches.insert(index, current_patch);
}

fn diff_children(
    old_children: &Vec<Rc<VirtualNode>>,
    new_children: &Vec<Rc<VirtualNode>>,
    index: usize,
    patches: &mut HashMap<usize, Vec<Patch>>,
    current_patch: &mut Vec<Patch>,
) {
    let diffs = list_diff(old_children, new_children);

    let mut left_node: Option<Rc<VirtualNode>> = None;
    let mut current_node_index: usize = index;

    for (index, child) in old_children.iter().enumerate() {
        let new_child = new_children.get(index);
        let offset = if let Some(node) = left_node {
            node.count
        } else {
            0
        };

        current_node_index = current_node_index + 1 + offset;
        dfs_walker(child, new_child, current_node_index, patches);
        left_node = Some(Rc::clone(child));
    }
}

enum DiffType {
    InsertAfter(Rc<VirtualNode>, Option<Rc<VirtualNode>>),
    Move(Rc<VirtualNode>, Option<Rc<VirtualNode>>),
    Remove(Rc<VirtualNode>),
}

fn list_diff(old_list: &Vec<Rc<VirtualNode>>, new_list: &Vec<Rc<VirtualNode>>) -> Vec<DiffType> {
    let mut diffs: Vec<DiffType> = vec![];
    let mut last_index: usize = 0;
    let mut last_placed_node: Option<Rc<VirtualNode>> = None;

    for (_, node) in new_list.iter().enumerate() {
        if let Some(index) = old_list.iter().position(|x| node.key == x.key) {
            if index < last_index {
                diffs.push(DiffType::Move(Rc::clone(node), last_placed_node));
            };
            last_index = if index > last_index {
                index
            } else {
                last_index
            };
        } else {
            diffs.push(DiffType::InsertAfter(Rc::clone(node), last_placed_node));
        };
        last_placed_node = Some(Rc::clone(node));
    }

    for (_, node) in old_list.iter().enumerate() {
        if new_list.iter().position(|x| node.key == x.key).is_none() {
            diffs.push(DiffType::Remove(Rc::clone(node)));
        };
    }

    diffs
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::virtual_node::{VirtualNode, list_diff};

    fn create_10_virtual_nodes() -> Vec<Rc<VirtualNode>> {
        let iter: Vec<usize> = (0..9).collect();

        iter.iter()
            .map(|&x| {
                Rc::new(VirtualNode::create(
                    format!("{}", x).as_str(),
                    String::from("p").as_str(),
                    format!("{}", x).as_str(),
                    vec![],
                ))
            })
            .collect()
    }

    fn create_list(pool: &Vec<Rc<VirtualNode>>, indexes: Vec<usize>) -> Vec<Rc<VirtualNode>> {
        indexes.iter().map(|&x| Rc::clone(&pool[x])).collect()
    }

    #[test]
    fn test_list_diff() {
        let pool = create_10_virtual_nodes();
        let old_list = create_list(&pool, vec![1, 2, 3, 4, 5]);
        let new_list = create_list(&pool, vec![2, 1, 7, 5, 4]);

        assert_eq!(list_diff(&old_list, &new_list).len(), 4);
    }
}
