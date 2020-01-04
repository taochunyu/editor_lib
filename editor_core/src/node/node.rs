use crate::node::content::Content;
use crate::node::replace::replace;
use crate::position::resolved_position::ResolvedPosition;
use crate::schema::node_type::NodeType;
use crate::slice::slice::Slice;
use std::rc::Rc;

pub struct Node {
    node_type: Rc<NodeType>,
    content: Rc<Content>,
}

impl Node {
    pub fn child(&self, index: usize) -> Result<&Rc<Self>, String> {
        self.content.get(index)
    }
    pub fn content(&self) -> &Rc<Content> {
        &self.content
    }
    pub fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        if from == 0 && to == self.content.size() {
            Ok(Rc::clone(&self))
        } else {
            let content = self.content.clone().cut(from, to)?;

            Ok(self.with_content(content))
        }
    }
    pub fn is_text(&self) -> bool {
        self.node_type.is_text()
    }
    pub fn node_type(&self) -> &Rc<NodeType> {
        &self.node_type
    }
    pub fn replace(
        self: Rc<Self>,
        from: usize,
        to: usize,
        slice: Slice,
    ) -> Result<Rc<Self>, String> {
        replace(
            self.clone().resolve(from)?,
            self.clone().resolve(to)?,
            slice,
        )
    }
    pub fn resolve(self: Rc<Self>, position: usize) -> Result<ResolvedPosition, String> {
        ResolvedPosition::resolve(&self, position)
    }
    pub fn same_markup(&self, other: &Rc<Node>) -> bool {
        false
    }
    pub fn size(&self) -> usize {
        self.content.size() + self.node_type.border_size()
    }
    pub fn with_content(self: Rc<Self>, content: Rc<Content>) -> Rc<Self> {
        if Rc::ptr_eq(&self.content, &content) {
            Rc::clone(&self)
        } else {
            Rc::new(Self {
                node_type: Rc::clone(&self.node_type),
                content: Rc::clone(&content),
            })
        }
    }

    pub fn new(node_type: Rc<NodeType>, node_content: Rc<Content>) -> Node {
        Node {
            node_type,
            content: node_content,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
