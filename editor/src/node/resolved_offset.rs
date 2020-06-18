use std::rc::Rc;
use std::cmp::Ordering;
use crate::node::Node;

type Index = usize;
type Offset = usize;

pub struct ResolvedOffset {
    offset: usize,
    path: Vec<(Rc<dyn Node>, Index, Offset)>,
}

fn find_path(
    previous: &mut Vec<(Rc<dyn Node>, usize, usize)>,
    current: Rc<dyn Node>,
    offset: usize
) -> Result<(), String> {
    if let Some(_) = current.as_text() {
        previous.push((current.clone(), 0, offset));
    } else {
        match offset.cmp(&current.content_size()) {
            Ordering::Greater => {
                return Err(format!("Offset {} outside of base node.", offset));
            },
            Ordering::Equal => {},
            Ordering::Less => {
                let index = current.find_index(offset)?;

                previous.push((current.clone(), index, offset));

                let next = current.get_child(index)?;
                let size = current.get_child_range(0..index)?.iter()
                    .fold(0, |acc, x| acc + x.size());
                let next_offset: usize = offset - size;

                find_path(previous, next, next_offset)?;
            }
        }
    };

    Ok(())
}

impl ResolvedOffset {
    fn resolve(base: Rc<dyn Node>, offset: usize) -> Result<Rc<Self>, String> {
        let mut path: Vec<(Rc<dyn Node>, Index, Offset)> = vec![];

        find_path(&mut path, base, offset)?;

        Ok(Rc::new(Self { path, offset }))
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn depth(&self) -> usize {
        self.path.len()
    }

    fn parent(&self) -> Option<Rc<dyn Node>> {
        Some(self.path.last()?.0.clone())
    }

    fn before(&self) -> Option<Rc<dyn Node>> {
        let parent = self.parent()?;
    }

    fn index(&self) -> usize {
        0
    }
}