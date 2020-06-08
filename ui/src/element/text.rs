use crate::element::{ElementType, ElementDescription, ElementResult};

pub struct TextProps {
    text: String,
}

pub struct Text {
    text: String,
}

impl ElementType for Text {
    type Properties = TextProps;

    fn name() -> String {
        String::from("text")
    }

    fn new(props: TextProps) -> ElementResult<ElementDescription> {
        ElementResult::Ok(ElementDescription::Text(Text {
            text: props.text,
        }))
    }

    fn to_instruction_description(&self) -> Vec<String> {
        vec![Self::name(), self.text.clone()]
    }
}
