use crate::core::action::{Message, Event};
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
        let mut result = String::from("<span data-marks=\"");
        match &self.mark_list {
            None => {}
            Some(marks) => {
                let mut divide = "";
                for mark in marks {
                    result.push_str(divide);
                    match mark {
                        Mark::Strong => result.push_str("strong"),
                        Mark::Em => result.push_str("em"),
                    }
                    divide = " ";
                }
            }
        };

        result.push_str("\">");
        result.push_str(self.text_content.as_str());
        result.push_str("</span>");
        result
    }
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node> {
        let event = &msg.event;
        let from_parent_offset = msg.resolved_from.parent_offset;
        let to_parent_offset = msg.resolved_to.parent_offset;

        match event {
            Event::KeyPress(text) => {
                Box::new(TextNode {
                    mark_list: None,
                    text_content: format!(
                        "{}{}{}",
                        self.text_content.get(0..from_parent_offset).unwrap_or(""),
                        text,
                        self.text_content.get(to_parent_offset..self.size()).unwrap_or(""),
                    ),
                })
            }
            _ => { self }
        }
    }
}