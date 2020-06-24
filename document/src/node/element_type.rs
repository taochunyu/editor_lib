use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;

pub trait ElementType: Sized + 'static {
    type Attributes;

    fn new() -> Rc<Self>;

    fn create_element(
        self: Rc<Self>,
        attrs: Rc<Self::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>
    ) -> Rc<dyn Node> {
        let element_children = match children {
            Some(nodes) => Some(Rc::new(Fragment::from(nodes))),
            None => None,
        };

        Element::new(self.clone(), attrs, element_children)
    }
}
