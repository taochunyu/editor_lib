use crate::node::Node;
use std::rc::Rc;
use std::any::Any;

pub struct Text {
    base: Rc<dyn Node>,
    content: String,
}

impl Node for Text {
    fn size(&self) -> usize {
        self.content_size()
    }

    fn content_size(&self) -> usize {
        self.content.len()
    }

    fn child_count(&self) -> usize {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        match self.content.get(from..to) {
            Some(sub) => {
                Ok(Rc::new(Self {
                    base: self.base.clone(),
                    content: String::from(sub),
                }))
            },
            None => Err(format!("rang {}..{} outside of text", from, to))
        }
    }

    fn index(&self, _: usize) -> Result<usize, String> {
        Err(format!("Cannot find offset index on text node."))
    }

    fn get_child(&self, _: usize) -> Result<Rc<dyn Node>, String> {
        Err(format!("Cannot get child on text node."))
    }
}

impl Text {
    pub(crate) fn new(base: Rc<dyn Node>, content: String) -> Rc<dyn Node> {
        Rc::new(Self {
            base,
            content,
        })
    }

    pub(crate) fn try_concat(&self, node: &Text) -> Option<Rc<dyn Node>> {
        if true {
            let text = format!("{}{}", self.content, node.content);

            Some(Rc::new(Self {
                base: self.base.clone(),
                content: text,
            }))
        } else {
            None
        }
    }
}
