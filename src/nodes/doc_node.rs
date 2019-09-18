use crate::core::node::Node;

pub struct DocNode {
    pub children: Vec<Box<dyn Node>>,
}

impl Node for DocNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        if self.children.len() == 0 {
            None
        } else {
            Some(&self.children)
        }
    }
    fn size(&self) -> usize {
        self.children.iter().fold(0, |acc, x| acc + x.size())
    }
    fn to_string(&self) -> String {
        let mut result = String::from("<div class=\"editor\">");
        for child in &self.children {
            result.push_str(child.to_string().as_str());
        }
        result.push_str("</div>");
        result
    }
}
