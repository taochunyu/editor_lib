use crate::core::node::Node;

pub struct ImageNode {
    src: String,
}

impl Node for ImageNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> { None }
    fn size(&self) -> usize { 1 }
    fn to_string(&self) -> String {
        format!("<img src=\"{}\">", self.src)
    }
}
