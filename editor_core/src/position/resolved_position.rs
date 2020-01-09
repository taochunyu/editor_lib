use crate::node::node::Node;
use std::rc::Rc;

pub struct ResolvedPosition {
    position: usize,
    path: Vec<(Rc<Node>, usize, usize)>,
    parent_offset: usize,
    depth: usize,
}

impl ResolvedPosition {
    pub fn position(&self) -> usize {
        self.position
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn index(&self, depth: usize) -> Result<usize, String> {
        match self.path.get(depth) {
            Some(node) => Ok(node.1),
            None => Err(format!("E32856441 {}", depth)),
        }
    }
    pub fn node(&self, depth: usize) -> Result<Rc<Node>, String> {
        match self.path.get(depth) {
            Some(node) => Ok(Rc::clone(&node.0)),
            None => Err(format!("E21889323 {}", depth)),
        }
    }
    pub fn node_after(&self) -> Result<Option<Rc<Node>>, String> {
        let parent = self.parent()?;
        let index = self.index(self.depth)?;

        if index == parent.content().count() {
            Ok(None)
        } else {
            let text_offset = self.text_offset()?;
            let node = parent.child(index)?;

            if text_offset == 0 {
                Ok(Some(Rc::clone(node)))
            } else {
                let n = Node::cut(node, text_offset, node.size())?;

                Ok(Some(n))
            }
        }
    }
    pub fn node_before(&self) -> Result<Option<Rc<Node>>, String> {
        let index = self.index(self.depth)?;
        let text_offset = self.text_offset()?;

        match (text_offset, index) {
            (0, 0) => Ok(None),
            (0, _) => {
                let parent = self.parent()?;
                let node = parent.child(index - 1)?;

                Ok(Some(Rc::clone(node)))
            }
            _ => {
                let parent = self.parent()?;
                let node = parent.child(index)?;
                let cut = Node::cut(node, 0, text_offset)?;

                Ok(Some(cut))
            }
        }
    }
    pub fn parent(&self) -> Result<Rc<Node>, String> {
        self.node(self.depth)
    }
    pub fn parent_offset(&self) -> usize {
        self.parent_offset
    }
    pub fn text_offset(&self) -> Result<usize, String> {
        match self.path.last() {
            Some(n) => Ok(self.position - n.2),
            None => Err(format!("ResolvedPosition: path is empty")),
        }
    }

    pub fn resolve(base: &Rc<Node>, position: usize) -> Result<ResolvedPosition, String> {
        if position > base.content().size() {
            return Err(format!("E63887706 {}", position));
        }
        if base.is_text() {
            return Err(format!("E33093981"))
        }

        let mut path: Vec<(Rc<Node>, usize, usize)> = vec![];
        let mut start: usize = 0;
        let mut parent_offset: usize = position;
        let mut cursor: &Rc<Node> = base;

        loop {
            let (index, offset) = cursor.content().find_index(parent_offset, false)?;
            let rem = parent_offset - offset;

            path.push((Rc::clone(&cursor), index, start + offset));

            if rem == 0 {
                break;
            }

            cursor = cursor.child(index)?;

            if cursor.is_text() {
                break;
            }

            parent_offset = rem - 1;
            start += offset + 1;
        }

        Ok(ResolvedPosition {
            depth: path.len() - 1,
            position,
            path,
            parent_offset,
        })
    }
}

#[cfg(test)]
mod tests {
}
