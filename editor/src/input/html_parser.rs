use std::rc::Rc;
use syd::ops::Fn;
use renderer::html::any_node::HTMLAnyNode;
use crate::node::Node;
use crate::schema::node_type::NodeType;

fn create_parser_rule<T: NodeType>() -> Box<dyn Fn(Box<dyn HTMLAnyNode>) -> Rc<dyn Node>> {
    unimplemented!()
}

pub struct HTMLParser {

}
