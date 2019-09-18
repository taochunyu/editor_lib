use crate::core::node::Node;

pub enum Mark {
    Strong,
    Em,
}

pub struct TextNode {
    pub mark_list: Option<Vec<Mark>>,
    pub text_content: String,
}

impl Node for TextNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        None
    }
    fn size(&self) -> usize {
        self.text_content.len()
    }
    fn to_string(&self) -> String {
        let mut mark_list_string = String::from("");
        match &self.mark_list {
            None => {}
            Some(marks) => {
                let mut divide = "";
                for mark in marks {
                    mark_list_string.push_str(divide);
                    match mark {
                        Mark::Strong => mark_list_string.push_str("strong"),
                        Mark::Em => mark_list_string.push_str("em"),
                    }
                    divide = " ";
                }
            }
        };

        format!("<span data-marks=\"{}\">{}</span>", mark_list_string, self.text_content)
    }
}
