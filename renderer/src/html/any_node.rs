pub enum HTMLNodeType {
    Element,
    Text,
    Other,
}

pub trait HTMLAnyNode {
    fn node_type(&self) -> HTMLNodeType;
    fn tag_name(&self) -> Option<String>;
    fn text_content(&self) -> Option<String>;
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn children(&self) -> Option<Vec<Box<dyn HTMLAnyNode>>>;
}
