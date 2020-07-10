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
    offset: usize,
    path: Vec<Step>,
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

        build_path(&mut path, base, offset)?;

        Ok(Rc::new(Self { path, offset }))
    }

    pub fn base(&self) -> Option<Rc<dyn Node>> {
        let path_node = self.path.first()?;

        Some(path_node.node.clone())
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn depth(&self) -> usize {
        self.path.len()
    }

    pub fn parent(&self) -> Option<Rc<dyn Node>> {
        Some(self.path.last()?.node.clone())
    }

    pub fn parent_offset(&self) -> Option<usize> {
        Some(self.path.last()?.offset)
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
            .map(|x| format!("({}, {}, {}),", x.node.to_html_string(), x.index, x.offset))
            .collect::<Vec<String>>()
            .join("\n")
    }
}