pub mod core;
pub mod nodes;
pub mod prelude {
    use crate::nodes::text_node::{TextNode, Mark};
    use crate::nodes::paragraph_node::{Align, ParagraphNode};
    use crate::core::node::Node;

    pub fn build_paragraph(content: &str) -> Box<dyn Node> {
        let text_node_box: Box<dyn Node> = Box::new(TextNode {
            mark_list: Some(vec![Mark::Strong]),
            text_content: String::from(content),
        });

        Box::new(ParagraphNode {
            align: Align::Left,
            children: vec![text_node_box],
        })
    }
}