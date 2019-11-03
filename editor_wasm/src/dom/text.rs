struct Text {
    text: web_sys::Text,
}

impl From<String> for Text {
    fn from(content: String) -> Text {
        let text = web_sys::Text::new_with_data(content.as_str()).unwrap();
        
        Text { text }
    }
}