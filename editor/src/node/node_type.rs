use std::rc::Rc;
use std::any::Any;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::base::Base;
use crate::node::text::Text;

pub trait NodeType: Sized + 'static {
    type Attributes;

    fn new() -> Rc<Self>;

    fn create_node(
        self: Rc<Self>,
        attrs: Rc<Self::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>
    ) -> Rc<dyn Node> {
        let element_node_children = match children {
            Some(nodes) => Some(Rc::new(Fragment::from(nodes))),
            None => None,
        };

        Base::new(self.clone(), attrs, element_node_children)
    }

    fn create_text_node(
        self: Rc<Self>,
        attrs: Rc<Self::Attributes>,
        content: &str,
    ) -> Rc<dyn Node> {
        let base = Base::new(self.clone(), attrs, None);

        Text::new(base, String::from(content))
    }
}
