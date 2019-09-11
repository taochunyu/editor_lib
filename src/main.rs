use editor_core_rust::prelude::*;
use editor_core_rust::core::node::{ResolvedPosition, Node, resolve_position};
use editor_core_rust::nodes::doc_node::DocNode;
use editor_core_rust::core::action::{Message, Event};

fn main() {
    let doc: Box<dyn Node> = Box::new(DocNode {
        children: vec![build_paragraph("hi"), build_paragraph("hello")],
    });
    let msg = {
        let resolved_from = resolve_position(&doc, 8).unwrap_or(ResolvedPosition::default());
        let resolved_to = resolve_position(&doc, 8).unwrap_or(ResolvedPosition::default());

        Message {
            resolved_from,
            resolved_to,
            event: Event::KeyPress(String::from("u")),
        }
    };


    println!("{}", doc.update(&msg, 0).to_string());
}
