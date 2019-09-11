use crate::core::action::Message;
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
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node> {
        let from = msg.resolved_from.position;
        let to = msg.resolved_to.position;
        let mut start: usize = 0;
        let mut temp: Vec<Box<dyn Node>> = vec![];

        for child in self.children {
            let end = start + child.size();

            if start > to || end < from {
                temp.push(child);
            } else {
                temp.push(child.update(msg, depth + 1));
            }

            start = end;
        }

        Box::new(DocNode {
            children: temp,
        })
    }
}