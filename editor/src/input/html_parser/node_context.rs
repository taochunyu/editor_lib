use std::rc::Rc;
use std::ops::Fn;
use renderer::html::any_node::{HTMLAnyNode, HTMLNodeType};
use crate::node::Node;
use crate::schema::Schema;
use crate::schema::node_type::NodeType;
use crate::editor::Editor;
use crate::node::slice::Slice;
use crate::input::html_parser::Parse;
use crate::node::fragment::Fragment;

pub struct NodeContext<'a> {
    parse: Option<&'a Parse>,
    dom: Option<Box<dyn HTMLAnyNode>>,
    content: Vec<Rc<dyn Node>>,
}

impl<'a> Default for NodeContext<'a> {
    fn default() -> Self {
        Self {
            parse: None,
            dom: None,
            content: vec![],
        }
    }
}

pub enum NodeContextFinishResult {
    Node(Rc<dyn Node>),
    Nodes(Rc<Fragment>),
}

impl<'a> NodeContext<'a> {
    pub fn new(parse: &'a Parse, dom: Box<dyn HTMLAnyNode>) -> Self {
        Self {
            parse: Some(parse),
            dom: Some(dom),
            content: vec![],
        }
    }

    pub fn content_push(&mut self, node: Rc<dyn Node>) {
        self.content.push(node);
    }

    pub fn finish(&self) -> NodeContextFinishResult {
        match (self.parse, &self.dom) {
            (Some(parse), Some(dom)) => {
                let node = (parse)(dom, None);

                NodeContextFinishResult::Node(node)
            },
            _ => {
                let nodes = Rc::new(Fragment::from(self.content.clone()));

                NodeContextFinishResult::Nodes(nodes)
            },
        }
    }
}
