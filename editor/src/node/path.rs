use std::rc::Rc;
use std::cmp::Ordering;
use crate::node::Node;

#[derive(Clone)]
pub struct Step {
    pub node: Rc<dyn Node>,
    pub index: usize,
    pub offset: usize,
}

pub struct Path {
    base: Rc<dyn Node>,
    offset: usize,
    path: Vec<Step>,
    depth: usize,
    parent_offset: usize,
}

fn build_path(path: &mut Vec<Step>, node: Rc<dyn Node>, offset: usize, cursor: usize) -> Result<usize, String> {
    if !node.is_text() {
        match offset.cmp(&node.content_size()) {
            Ordering::Greater => {
                return Err(format!("Offset {} outside of base node.", offset));
            },
            Ordering::Equal => {
                path.push(Step {
                    node: node.clone(),
                    index: node.child_count(),
                    offset: node.content_size(),
                });
            },
            Ordering::Less => {
                let index = node.index(offset)?;
                let child = node.get_child(index)?;
                let size = node.get_child_range(0..index)?.iter()
                    .fold(0, |acc, x| acc + x.size());
                let cursor = cursor + size;

                path.push(Step {
                    node: node.clone(),
                    index,
                    offset: cursor,
                });

                if offset != size && !child.is_text() {
                    let offset = offset - size - 1;

                    return build_path(path, child, offset, cursor + 1);
                }

            }
        }
    }

    Ok(offset)
}

impl Path {
    pub(crate) fn new(base: Rc<dyn Node>, offset: usize) -> Result<Rc<Self>, String> {
        if base.is_text() {
            return Err(format!("Path base node cannot be a text node."))
        }

        let mut path: Vec<Step> = vec![];

        let parent_offset = build_path(&mut path, base.clone(), offset, 0)?;

        let path_length = path.len();

        if path_length == 0 {
            Err(format!("Path must contain base node."))
        } else {
            Ok(Rc::new(Self { path, offset, base, parent_offset, depth: path_length - 1 }))
        }
    }

    pub fn base(&self) -> Rc<dyn Node> {
        self.base.clone()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn parent(&self) -> Rc<dyn Node> {
        match self.path.last() {
            Some(parent) => parent.node.clone(),
            None => self.base.clone(),
        }
    }

    pub fn parent_offset(&self) -> usize {
        self.parent_offset
    }

    pub fn text_offset(&self) -> usize {
        match self.path.last() {
            Some(parent) => self.offset - parent.offset,
            None => 0,
        }
    }

    pub fn node_before(&self) -> Option<Rc<dyn Node>> {
        let step = self.path.last()?;
        let text_offset = self.text_offset();

        if text_offset != 0 {
            match step.node.clone().cut(0, text_offset) {
                Ok(node) => Some(node),
                Err(_) => None,
            }
        } else if step.index == 0 {
            None
        } else {
            match step.node.clone().get_child(step.index - 1) {
                Ok(node) => Some(node),
                Err(_) => None,
            }
        }
    }

    pub fn node_after(&self) -> Option<Rc<dyn Node>> {
        let step = self.path.last()?;

        if step.index == step.node.child_count() {
            None
        } else {
            match step.node.clone().get_child(step.index) {
                Ok(node) => {
                    let text_offset = self.text_offset();

                    if text_offset == 0 {
                        Some(node)
                    } else {
                        match step.node.clone().cut(text_offset, node.size()) {
                            Ok(node) => Some(node),
                            Err(_) => None,
                        }
                    }
                },
                Err(_) => None,
            }
        }
    }

    pub fn step(&self, depth: usize) -> Result<Step, String> {
        match self.path.get(depth) {
            Some(path_node) => Ok(path_node.clone()),
            None => Err(format!("Depth {} out range of offset path", depth)),
        }
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::test::tools::create_root;
    use crate::node::path::Path;

    fn to_debug_string(path: Rc<Path>) -> String {
        path.path.iter()
            .map(|x| format!("    ({}, {}, {}),", x.node.type_name(), x.index, x.offset))
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn build_path() {
        let base = create_root();
        let path = Path::new(base, 14).unwrap();

        println!("Path Debug String: [\n{}\n]", to_debug_string(path));
    }
}