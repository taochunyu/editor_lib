mod node_context;
mod parse_context;

use std::rc::Rc;
use std::ops::Fn;
use std::cell::RefCell;
use renderer::html::any_node::{HTMLAnyNode, HTMLNodeType};
use crate::node::Node;
use crate::schema::Schema;
use crate::schema::node_type::NodeType;
use crate::editor::Editor;
use crate::node::slice::Slice;
use crate::input::html_parser::parse_context::ParseContext;

type Match = Box<dyn Fn(&Box<dyn HTMLAnyNode>) -> bool>;
type Parse = Box<dyn Fn(&Box<dyn HTMLAnyNode>, Option<Vec<Rc<dyn Node>>>) -> Rc<dyn Node>>;
type ParseRules = Vec<(Match, Parse)>;

pub struct HTMLParser {
    schema: Rc<RefCell<Schema>>,
    rules: ParseRules,
}

pub enum ParseResult {
    Node(Rc<dyn Node>),
}

impl HTMLParser {
    pub fn new(schema: Rc<RefCell<Schema>>) -> Self {
        Self { schema, rules: vec![] }
    }

    pub fn add_rules<T: NodeType>(&mut self) {
        for rule in T::parse_from_html() {
            let schema = self.schema.clone();
            let (m, p) = rule;
            let match_: Match = Box::new(move |node| m(node));
            let parse_: Parse = Box::new(move |node, children| {
                let attrs: Rc<T::Attributes> = p(node);

                schema.borrow().create_node::<T>(attrs, children)
            });

            self.rules.push((match_, parse_));
        }
    }

    pub fn parse(&self, root: Box<dyn HTMLAnyNode>) -> ParseResult {
        let mut parse_context = ParseContext::new(self.schema.clone());

        parse_context.add_node(root);

        ParseResult::Node(parse_context.finish())
    }
}