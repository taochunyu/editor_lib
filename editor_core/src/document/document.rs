use std::rc::Rc;
use crate::node::node::Node;
use crate::slice::slice::Slice;
use crate::document::replace::replace;
use crate::position::resolved_position::ResolvedPosition;

struct Document {
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
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use crate::document::document::Document;
    use crate::node::node::tests::{mock_text_node, mock_container_node};
    use crate::node::content::Content;
    use crate::slice::slice::Slice;

    fn create_document_1() -> Document {
        let text = mock_text_node("hello");
        let paragraph = mock_container_node("paragraph", Content::from(Rc::new(text)));
        let doc = mock_container_node("document", Content::from(Rc::new(paragraph)));

        Document {
            root: Rc::new(doc),
        }
    }

    fn create_document_2() -> Document {
        let text_1 = mock_text_node("hi");
        let text_2 = mock_text_node("hello");
        let paragraph_1 = mock_container_node("paragraph", Content::from(Rc::new(text_1)));
        let paragraph_2 = mock_container_node("paragraph", Content::from(Rc::new(text_2)));
        let doc = mock_container_node(
            "document",
            Content::from(vec![Rc::new(paragraph_1), Rc::new(paragraph_2)]));

        Document {
            root: Rc::new(doc),
        }
    }

    #[test]
    fn test_replace() {
        let document_1 = create_document_1();

        match document_1.replace(2, 5, Slice::new()) {
            Ok(d) => assert_eq!(d.root().to_string(), String::from("(document, ,[(paragraph, ,[(text, ,\"ho\")])])")),
            Err(e) => assert_eq!(e, String::from("")),
        }

        let document_2 = create_document_2();

        match document_2.replace(2, 9, Slice::new()) {
            Ok(d) => assert_eq!(d.root().to_string(), String::from("(document, ,[(paragraph, ,[(text, ,\"ho\")])])")),
            Err(e) => assert_eq!(e, String::from("")),
        }
    }
}
