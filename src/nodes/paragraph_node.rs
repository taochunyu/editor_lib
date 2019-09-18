use crate::core::node::Node;

pub enum Align {
    Left,
    Center,
    Right,
}

pub struct ParagraphNode {
    pub align: Align,
    pub children: Vec<Box<dyn Node>>,
}

impl Node for ParagraphNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        if self.children.len() == 0 {
            None
        } else {
            Some(&self.children)
        }
    }
    fn size(&self) -> usize {
        self.children.iter().fold(0, |acc, x| acc + x.size()) + 2
    }
    fn to_string(&self) -> String {
        let mut result = String::from("<p data-align=\"");
        match self.align {
            Align::Left => result.push_str("left"),
            Align::Center => result.push_str("center"),
            Align::Right => result.push_str("right"),
        };
        result.push_str("\">");
        for child in &self.children {
            result.push_str(child.to_string().as_str());
        }
        result.push_str("</p>");
        result
    }
}
