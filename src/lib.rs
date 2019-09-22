pub mod core;
pub mod nodes;
pub mod pre {
    use crate::core::model::fragment::Fragment;
    use crate::core::model::node::TreeNode;
    use crate::core::model::replace::replace;
    use crate::core::model::slice::Slice;
    use crate::nodes::doc_node::DocNode;
    use crate::nodes::paragraph_node::{Align, ParagraphNode};
    use crate::nodes::text_node::TextNode;
    use std::rc::Rc;

    fn mk_text(text: &str) -> Rc<TreeNode> {
        Rc::new(TreeNode {
            content: None,
            node: Rc::new(TextNode {
                mark_list: None,
                text_content: String::from(text),
            }),
        })
    }
    fn mk_paragraph(inline_nodes: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
        let size = inline_nodes.iter().fold(0, |acc, x| acc + x.size());

        Rc::new(TreeNode {
            content: Some(Rc::new(Fragment::new(inline_nodes, size))),
            node: Rc::new((ParagraphNode { align: Align::Left })),
        })
    }
    fn mk_doc(block_nodes: Vec<Rc<TreeNode>>) -> Rc<TreeNode> {
        let size = block_nodes.iter().fold(0, |acc, x| acc + x.size());
        Rc::new(TreeNode {
            content: Some(Rc::new(Fragment::new(block_nodes, size))),
            node: Rc::new(DocNode),
        })
    }

    pub fn one_test() {
        let doc = mk_doc(vec![
            mk_paragraph(vec![mk_text("hi")]),
            mk_paragraph(vec![mk_text("hello")]),
        ]);
        let slice_content = vec![mk_text("a")];
        let slice = Slice::new(Rc::new(Fragment::new(slice_content, 1)), 0, 0);

        match replace(doc, 8, 8, slice) {
            Ok(new_doc) => println!("ok: {}", new_doc.to_string()),
            Err(msg) => println!("error: {}", msg),
        };
    }
}
