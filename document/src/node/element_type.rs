use std::rc::Rc;
use render_vm::DOM;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::node::element::Element;
use crate::view::View;

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

    fn render(view: Rc<View>, node: Rc<dyn Node>, state: Self::State) -> (OuterDOM, ContentDOM);
}
