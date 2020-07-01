use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use render_vm::DOM;

pub trait ElementType: Sized + 'static {
    type Attributes;

    fn name() -> &'static str;

    fn create(
        attrs: Rc<Self::Attributes>,
        children: Option<Vec<Rc<dyn Node>>>
    ) -> Rc<dyn Node> {
        let element_children = match children {
            Some(nodes) => Some(Rc::new(Fragment::from(nodes))),
            None => None,
        };

        Element::<Self>::new(attrs, element_children)
    }

    fn update(_dom: DOM, _old_node: Rc<dyn Node>, _new_node: Rc<dyn Node>) -> bool {
        true
    }

    fn render(dom: DOM, node: Rc<dyn Node>);
}
