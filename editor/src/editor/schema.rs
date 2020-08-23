use std::rc::Rc;
use crate::editor::Editor;
use crate::schema::node_type::NodeType;
use crate::node::Node;
use crate::node::text::Text;
use crate::node::element::Element;
use crate::node::fragment::Fragment;

impl Editor {
    pub fn create_node<T: NodeType>(
        &self,
        attrs: Rc<T::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>,
    ) -> Rc<dyn Node> {
        self.schema.borrow().create_node::<T>(attrs, children)
    }

    pub fn create_text_node(&self, content: &str) -> Rc<dyn Node> {
        self.schema.borrow().create_text_node(content)
    }
}
