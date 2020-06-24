use std::rc::Rc;
use crate::node::Node;
use crate::node::text::Text;

pub trait TextType: Sized + 'static {
    fn new() -> Rc<Self>;

    fn create_text(self: Rc<Self>, content: &str) -> Rc<dyn Node> {
        Text::new(String::from(content))
    }
}
