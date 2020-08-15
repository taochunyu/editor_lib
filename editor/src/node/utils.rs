use std::rc::Rc;
use crate::schema::node_type::NodeType;
use crate::node::Node;
use crate::node::text::Text;

pub fn create_element<T: NodeType>(
    attrs: T::Attributes,
    children: Option<Vec<Rc<dyn Node>>>,
) -> Rc<dyn Node> {
    T::create(Rc::new(attrs), children)
}

pub fn create_text(content: &str) -> Rc<dyn Node> {
    Text::new(String::from(content))
}
