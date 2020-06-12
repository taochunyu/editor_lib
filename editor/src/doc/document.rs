use std::rc::Rc;
use crate::n::node::Node;
use crate::slice::slice::Slice;
use crate::doc::replace::replace;
use crate::position::resolved_position::ResolvedPosition;
use crate::schema::node_type::NodeType;
use crate::n::content::Content;
use std::env::var;

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
        let doc_type = NodeType::new(String::from("doc"), String::from("1"));
        let paragraph_type = NodeType::new(String::from("paragraph"), String::from("1"));
        let text_type = NodeType::new(String::from("text"), String::from(""));

        let text_1 = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from("hello")))));
        let text_2 = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from("world")))));
        let paragraph_1 = Rc::new(paragraph_type.clone().create(Rc::new(Content::from(text_1))));
        let paragraph_2 = Rc::new(paragraph_type.clone().create(Rc::new(Content::from(text_2))));

        let mut content: Vec<Rc<Node>> = vec![paragraph_1, paragraph_2];

        for index in 0..10 {
            let text = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from("1234567890")))));
            let paragraph = Rc::new(paragraph_type.clone().create(Rc::new(Content::from(text))));

            content.push(paragraph);
        }

        let root = Rc::new(doc_type.clone().create(Rc::new(Content::from(content))));

        Rc::new(Self { root })
    }
}

#[cfg(test)]
mod test {
    use crate::doc::document::Document;
    use crate::slice::slice::Slice;

    #[test]
    fn test() {
        let doc = Document::new();

        match doc.replace(8, 8, Slice::text("the ")) {
            Ok(d) => assert_eq!(
                d.root().to_string(),
                "(doc, ,[(paragraph, ,[(text, ,\"hello\")]), (paragraph, ,[(text, ,\"the world\")])])"
            ),
            Err(e) => panic!(e),
        }
    }
}
