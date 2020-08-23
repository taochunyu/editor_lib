use std::rc::{Weak, Rc};
use renderer::html::any_node::HTMLAnyNode;
use crate::editor::Editor;
use crate::schema::node_type::NodeType;
use crate::input::html_parser::{ParseResult, HTMLParser};

impl Editor {
    // pub fn register_html_parser_rules<T: NodeType>(&mut self) -> &mut Self {
    //     self.html_parser.add_rules::<T>();
    //
    //     self
    // }

    pub fn parser(&self) -> Rc<HTMLParser> {
        self.html_parser.clone()
    }
}