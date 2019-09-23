use crate::core::model::node::{Node, TreeNode};
use std::rc::Rc;

#[derive(Copy, Clone)]
pub enum Mark {
    Strong,
    Em,
}

pub struct TextNode {
    pub mark_list: Option<Vec<Mark>>,
    pub text_content: String,
}

impl Node for TextNode {
    fn type_name(&self) -> String { String::from("text") }
    fn is_text(&self) -> bool {
        true
    }
    fn text_content(&self) -> &str {
        self.text_content.as_str()
    }
    fn slice_text_content(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        let mark_list = match &self.mark_list {
            None => None,
            Some(ml) => Some(ml.clone()),
        };
        Ok(Rc::new(TextNode {
            mark_list,
            text_content: String::from(&self.text_content[from..to]),
        }))
    }
    fn need_join(&self, other: &dyn Node) -> bool {
        Node::is_mark_same(self, other)
    }
    fn join(&self, other: &dyn Node) -> Result<Rc<dyn Node>, String> {
        let mark_list = match &self.mark_list {
            None => None,
            Some(ml) => Some(ml.clone()),
        };
        Ok(Rc::new(TextNode {
            mark_list,
            text_content: format!("{}{}", self.text_content, other.text_content()),
        }))
    }
    fn content_size(&self) -> usize {
        self.text_content.len()
    }
    fn to_string(&self, content: String) -> String {
        format!(
            "<span data-marks=\"{}\">{}</span>",
            self.mark_to_string(),
            self.text_content
        )
    }
    fn mark_to_string(&self) -> String {
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

        mark_list_string
    }
}
