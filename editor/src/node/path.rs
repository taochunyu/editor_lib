use std::rc::Rc;
use std::cmp::Ordering;
use crate::node::Node;

#[derive(Clone)]
struct PathNode {
    node: Rc<dyn Node>,
    index: usize,
    offset: usize,
}

pub struct ResolvedOffset {
    offset: usize,
    path: Vec<PathNode>,
}

fn build_path(
    previous: &mut Vec<PathNode>,
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
                    previous.push(PathNode {
                        node: current_node.clone(),
                        index: current_node.child_count(),
                        offset,
                    });
                },
                Ordering::Less => {
                    let index = current_node.index(offset)?;

                    previous.push(PathNode {
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

impl ResolvedOffset {
    pub(crate) fn new(base: Rc<dyn Node>, offset: usize) -> Result<Rc<Self>, String> {
        let mut path: Vec<PathNode> = vec![];

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
        let path_node = self.path.last()?;

        Some(path_node.node.clone())
    }

    pub fn before(&self) -> Option<Rc<dyn Node>> {
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

    pub fn after(&self) -> Option<Rc<dyn Node>> {
        let path_node = self.path.last()?;

        match path_node.node.get_child(path_node.index.clone() + 1) {
            Ok(node) => Some(node),
            Err(_) => None,
        }
    }

    fn path_node(&self, depth: usize) -> Result<PathNode, String> {
        match self.path.get(depth) {
            Some(path_node) => Ok(path_node.clone()),
            None => Err(format!("Depth {} out range of offset path", depth)),
        }
    }
}