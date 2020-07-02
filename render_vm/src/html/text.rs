use crate::html::NodeDescription;

pub struct Text {
    content: String,
}

impl NodeDescription for Text {
    type Attributes = String;

    fn name() -> &'static str {
        "text"
    }

    fn new(attrs: Self::Attributes) -> Self {
        Text { content: attrs }
    }

    fn to_instruction(&self) -> Vec<String> {
        vec![String::from(Self::name()), self.content.clone()]
    }
}