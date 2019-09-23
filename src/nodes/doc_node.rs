use crate::core::model::node::Node;

pub struct DocNode;

impl Node for DocNode {
    fn type_name(&self) -> String { String::from("doc") }
    fn to_string(&self, content: String) -> String {
        format!("<div class=\"editor\">{}</div>", content)
    }
    fn mark_to_string(&self) -> String {
        String::from("")
    }
}
