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
}

fn build_path(
    previous: &mut Vec<Step>,
    current_node: Rc<dyn Node>,
    offset: usize
) -> Result<(), String> {
    match current_node.as_text() {
        Some(_) => {},
        None => {
            match offset.cmp(&current_node.content_size()) {
                Ordering::Greater => {
                    return Err(format!("Offset {} outside of base node.", offset));
                },
                Ordering::Equal => {
                    previous.push(Step {
                        node: current_node.clone(),
                        index: current_node.child_count(),
                        offset,
                    });
                },
                Ordering::Less => {
                    let index = current_node.index(offset)?;

                    previous.push(Step {
                        node: current_node.clone(),
                        index,
                        offset,
                    });

                    let next_node = current_node.get_child(index)?;
                    let size = current_node.get_child_range(0..index)?.iter()
                        .fold(0, |acc, x| acc + x.size());
                    let next_offset: usize = offset - size - 1;

                    build_path(previous, next_node, next_offset)?;
                }
            }
        }
    };

    Ok(())
}

impl Path {
    pub(crate) fn new(base: Rc<dyn Node>, offset: usize) -> Result<Rc<Self>, String> {
        let mut path: Vec<Step> = vec![];

        build_path(&mut path, base.clone(), offset)?;

        let path_length = path.len();

        if path_length == 0 {
            Err(format!("Path must contain base node."))
        } else {
            Ok(Rc::new(Self { path, offset, base, depth: path_length - 1 }))
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
        match self.path.last() {
            Some(parent) => parent.offset,
            None => self.offset,
        }
    }

    pub fn node_before(&self) -> Option<Rc<dyn Node>> {
        let path_node = self.path.last()?;

        if path_node.index < 1 {
            None
        } else {
            match path_node.node.get_child(path_node.index.clone() - 1) {
                Ok(node) => Some(node),
                Err(_) => None,
            }
        }
    }

    pub fn node_after(&self) -> Option<Rc<dyn Node>> {
        let path_node = self.path.last()?;

        match path_node.node.get_child(path_node.index.clone() + 1) {
            Ok(node) => Some(node),
            Err(_) => None,
        }
    }

    pub fn step(&self, depth: usize) -> Result<Step, String> {
        match self.path.get(depth) {
            Some(path_node) => Ok(path_node.clone()),
            None => Err(format!("Depth {} out range of offset path", depth)),
        }
    }

    pub fn to_string(&self) -> String {
        self.path.iter()
            .map(|x| format!("({}, {}, {}),", x.node.serialize(), x.index, x.offset))
            .collect::<Vec<String>>()
            .join("\n")
    }
}