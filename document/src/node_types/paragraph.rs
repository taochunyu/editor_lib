use std::rc::Rc;
use render_vm::html::paragraph;
use crate::node::element_type::ElementType;
use crate::node::Node;
use crate::view::View;

pub struct Paragraph;

impl ElementType for Paragraph {
    type Attributes = ();
    type State = ();

    fn name() -> &'static str {
        "paragraph"
    }

    fn render(view: Rc<View>, node: Rc<dyn Node>, state: Self::State) -> (OuterDOM, ContentDOM) {
        let paragraph = view.ui().create_element::<paragraph::Paragraph>(());

        (paragraph, paragraph.clone())
    }
}
