use std::rc::Rc;
use crate::node::node::Node;
use crate::slice::slice::Slice;
use crate::document::replace::replace;
use crate::position::resolved_position::ResolvedPosition;
use crate::schema::node_type::NodeType;
use crate::node::content::Content;

pub struct Document {
    root: Rc<Node>,
}

impl Document {
    pub fn root(&self) -> &Rc<Node> {
        &self.root
    }
    pub fn replace(&self, from: usize, to: usize, slice: Slice) -> Result<Self, String> {
        Ok(Self {
            root: replace(self.resolve(from)?, self.resolve(to)?, slice)?,
        })
    }
    pub fn resolve(&self, position: usize) -> Result<ResolvedPosition, String> {
        ResolvedPosition::resolve(&self.root, position)
    }

    pub fn new() -> Rc<Self> {
        let doc_type = NodeType::new(String::from("doc"), String::from(""));
        let paragraph_type = NodeType::new(String::from("paragraph"), String::from(""));
        let text_type = NodeType::new(String::from("text"), String::from(""));

        let text_1 = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from("hello")))));
        let text_2 = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from("world")))));
        let paragraph_1 = Rc::new(paragraph_type.clone().create(Rc::new(Content::from(text_1))));
        let paragraph_2 = Rc::new(paragraph_type.clone().create(Rc::new(Content::from(text_2))));
        let root = Rc::new(doc_type.clone().create(Rc::new(Content::from(vec![paragraph_1, paragraph_2]))));

        Rc::new(Self { root })
    }
}

#[cfg(test)]
mod test {}
