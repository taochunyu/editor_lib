use crate::html::{Node, NodeDescription};

pub struct Text {
    content: String,
}

impl NodeDescription for Text {
    type Attributes = &'static str;

    fn name() -> &'static str {
        "text"
    }

    fn new(attrs: Self::Attributes) -> Result<Self, String> {
        Ok(Text { content: String::from(attrs) })
    }

    fn to_instruction(&self) -> Vec<String> {
        vec![String::from(Self::name()), self.content.clone()]
    }
}