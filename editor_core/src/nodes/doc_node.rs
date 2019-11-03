use crate::model::node::Node;
use std::rc::Rc;
use crate::view::virtual_node::VirtualNode;

pub struct DocNode;

impl Node for DocNode {
    fn type_name(&self) -> String {
        String::from("doc")
    }

    fn to_string(&self, content: String) -> String {
        format!("<div class=\"editor\">{}</div>", content)
    }

    fn mark_to_string(&self) -> String {
        String::from("")
    }

    fn render(&self, children: Vec<Rc<VirtualNode>>) -> Rc<VirtualNode> {
        Rc::new(VirtualNode::create(String::from("1"), String::from("div"), String::from(""), children))
    }
}
