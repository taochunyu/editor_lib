pub struct NodeType;

impl NodeType {
    pub fn border_size(&self) -> usize {
        2
    }
    pub fn is_text(&self) -> bool { false }

    pub fn new() -> NodeType {
        NodeType
    }
}
