use crate::node::Node;
use std::rc::Rc;

struct TextNode {
    text: String,
}

impl Node for TextNode {
    fn size(&self) -> usize {
        self.content_size()
    }

    fn content_size(&self) -> usize {
        self.text.len()
    }

    fn cut(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        match self.text.get(from..to) {
            Some(sub) => {
                Ok(Rc::new(Self { text: String::from(sub) }))
            },
            None => Err(format!("rang {}..{} outside of text", from, to))
        }
    }
}