pub mod mark_type;
pub mod node_type;

use std::collections::HashSet;
use std::any::TypeId;
use std::rc::Rc;
use crate::schema::node_type::NodeType;
use crate::node::Node;
use crate::node::element::Element;
use crate::node::text::Text;
use crate::node::fragment::Fragment;

pub struct SchemaUUID;

pub type SchemaToken = Rc<SchemaUUID>;

pub struct Schema {
    token: SchemaToken,
    nodes: HashSet<TypeId>,
}

impl Schema {
    pub fn new() -> Self {
        Self {
            token: Rc::new(SchemaUUID),
            nodes: HashSet::new(),
        }
    }

    pub fn register_node_type<T: NodeType>(&mut self) -> bool {
        let id = TypeId::of::<T>();

        self.nodes.insert(id)
    }

    pub fn has_node_type<T: NodeType>(&self) -> bool {
        let id = TypeId::of::<T>();

        self.nodes.get(&id).is_some()
    }

    pub fn create_node<T: NodeType>(
        &self,
        attrs: Rc<T::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>,
    ) -> Rc<dyn Node> {
        if !self.has_node_type::<T>() {
            panic!("NodeType isn't registered in schema.")
        }

        let element_children = match children {
            Some(nodes) => Some(Rc::new(Fragment::from(nodes))),
            None => None,
        };

        Element::<T>::new(attrs, element_children, self.token.clone())
    }

    pub fn create_text_node(&self, content: &str) -> Rc<dyn Node> {
        Text::new(String::from(content))
    }
}
