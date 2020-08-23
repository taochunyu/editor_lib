use std::rc::Rc;
use std::ops::Fn;
use std::cell::RefCell;
use renderer::html::any_node::{HTMLAnyNode, HTMLNodeType};
use crate::node::Node;
use crate::schema::Schema;
use crate::schema::node_type::NodeType;
use crate::editor::Editor;
use crate::node::slice::Slice;
use crate::input::html_parser::node_context::{NodeContext, NodeContextFinishResult};

pub struct ParseContext<'a> {
    schema: Rc<RefCell<Schema>>,
    nodes: Vec<NodeContext<'a>>,
}

impl<'a> ParseContext<'a> {
    pub fn new(schema: Rc<RefCell<Schema>>) -> Self {
        let top = NodeContext::default();

        Self { schema, nodes: vec![top] }
    }

    pub fn finish(&self) -> Rc<dyn Node> {
        match self.top().finish() {
            NodeContextFinishResult::Node(node) => node,
            NodeContextFinishResult::Nodes(nodes) => {
                nodes.get(0).unwrap()
            },
        }
    }

    pub fn add_node(&mut self, node: Box<dyn HTMLAnyNode>) {
        match node.node_type() {
            HTMLNodeType::Text => self.add_text_node(node),
            _ => {},
        }
    }

    pub fn add_text_node(&mut self, node: Box<dyn HTMLAnyNode>) {
        let text = node.text_content().unwrap_or(String::new());
        let text_node = self.schema.borrow().create_text_node(text.as_str());

        self.insert_node(text_node);
    }

    fn top(&self) -> &NodeContext<'a> {
        match self.nodes.first() {
            Some(top) => top,
            None => unreachable!("ParseContext has one node at least."),
        }
    }

    fn top_mut(&mut self) -> &mut NodeContext<'a> {
        match self.nodes.first_mut() {
            Some(top) => top,
            None => unreachable!("ParseContext has one node at least."),
        }
    }

    fn insert_node(&mut self, node: Rc<dyn Node>) {
        self.top_mut().content_push(node);
    }
}