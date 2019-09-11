use crate::core::action::Message;

pub trait Node {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>>;
    fn size(&self) -> usize;
    fn to_string(&self) -> String;
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node>;
}

pub struct ResolvedPosition {
    pub position: usize,
    pub path: Vec<(usize, usize)>,
    pub parent_offset: usize,
}

impl Default for ResolvedPosition {
    fn default() -> ResolvedPosition {
        ResolvedPosition {
            position: 0,
            path: vec![],
            parent_offset: 0,
        }
    }
}

pub fn resolve_position(
    node: &Box<dyn Node>,
    position: usize,
) -> Result<ResolvedPosition, &str> {
    if node.size() < position {
        return Err("Position out of Range.");
    }

    let mut path: Vec<(usize, usize)> = vec![];
    let mut cursor: Option<&Box<dyn Node>> = Some(node);
    let mut start: usize = 0;
    let mut parent_offset: usize = position;

    while let Some(parent) = cursor {
        match parent.children() {
            None => cursor = None,
            Some(children) => {
                let mut index = 0;
                let mut offset = 0;

                for child in children {
                    if offset + child.size() > parent_offset {
                        cursor = Some(child);
                        break;
                    }
                    index += 1;
                    offset += child.size();
                }

                path.push((index, start + offset));

                let rem: usize = parent_offset - offset;

                if rem == 0 {
                    break;
                }

                if cursor.is_some() && cursor.unwrap().children().is_none() {
                    break;
                }

                parent_offset = rem - 1;
                start += offset + 1;
            }
        }
    }

    Ok(ResolvedPosition {
        position,
        path,
        parent_offset,
    })
}