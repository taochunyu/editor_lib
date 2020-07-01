use std::rc::Rc;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use render_vm::DOM;

type OuterDOM = Rc<DOM>;
type ContentDOM = Option<Rc<DOM>>;

pub trait ElementType: Sized + 'static {
    type Attributes;
    type State;

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

    fn render(state: Self::State, node: Rc<dyn Node>) -> (OuterDOM, ContentDOM);
}
