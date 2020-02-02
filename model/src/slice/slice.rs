use crate::node::content::Content;
use std::rc::Rc;
use crate::schema::node_type::NodeType;

pub struct Slice {
    open_start: usize,
    open_end: usize,
    content: Rc<Content>,
}

impl Slice {
    pub fn content(&self) -> &Rc<Content> {
        &self.content
    }
    pub fn open_start(&self) -> usize {
        self.open_start
    }
    pub fn open_end(&self) -> usize {
        self.open_end
    }
    pub fn size(&self) -> usize {
        self.content.size() - self.open_start - self.open_end
    }

    pub fn new() -> Self {
        Self {
            open_start: 0,
            open_end: 0,
            content: Rc::new(Content::Nothing),
        }
    }

    pub fn text(text: &str) -> Self {
        let text_type = NodeType::new(String::from("text"), String::from(""));
        let text = Rc::new(text_type.clone().create(Rc::new(Content::from(String::from(text)))));

        Self {
            open_start: 0,
            open_end: 0,
            content: Rc::new(Content::from(text)),
        }
    }
}
