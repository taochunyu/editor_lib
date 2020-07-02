use std::rc::Rc;
use render_vm::html::div::Div;
use crate::node::element_type::ElementType;
use crate::node::Node;
use crate::view::View;

pub struct Root;

impl ElementType for Root {
    type Attributes = ();
    type State = ();

    fn name() -> &'static str {
        "root"
    }

    fn render(view: Rc<View>, node: Rc<dyn Node>, state: Self::State) -> (OuterDOM, ContentDOM) {
        let outer = view.ui().create_element::<Div>(());

        (outer, outer.clone())
    }
}
