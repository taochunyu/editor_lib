use crate::model::node::Node;
use crate::view::virtual_node::VirtualNode;
use std::rc::Rc;

pub enum Align {
    Left,
    Center,
    Right,
}

pub struct ParagraphNode {
    pub align: Align,
}

impl Node for ParagraphNode {
    fn type_name(&self) -> String {
        String::from("paragraph")
    }
    fn to_string(&self, content: String) -> String {
        format!(
            "<p data-align=\"{}\">{}</p>",
            self.mark_to_string(),
            content
        )
    }
    fn mark_to_string(&self) -> String {
        String::from(match self.align {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
        })
    }
    fn render(&self, children: Vec<Rc<VirtualNode>>) -> Rc<VirtualNode> {
        Rc::new(VirtualNode::create(String::from("2"), String::from("p"), String::from(""), children))
    }
}
