use crate::node::Node;
use std::rc::Rc;
use std::any::Any;

pub struct TextNode {
    marks: Vec<u64>,
    text: String,
}

impl Node for TextNode {
    fn size(&self) -> usize {
        self.content_size()
    }

    fn content_size(&self) -> usize {
        self.text.len()
    }

    fn child_count(&self) -> usize {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        match self.text.get(from..to) {
            Some(sub) => {
                Ok(Rc::new(Self { marks: vec![], text: String::from(sub) }))
            },
            None => Err(format!("rang {}..{} outside of text", from, to))
        }
    }

    fn find_index(&self, _: usize) -> Result<usize, String> {
        Err(format!("Cannot find offset index on text node."))
    }

    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String> {
        Err(format!("Cannot get child on text node."))
    }
}

impl TextNode {
    pub(crate) fn try_concat(&self, node: &TextNode) -> Option<Rc<dyn Node>> {
        if self.marks.len() == node.marks.len() {
            let text = format!("{}{}", self.text, node.text);

            Some(Rc::new(Self { marks: vec![], text }))
        } else {
            None
        }
    }
}
