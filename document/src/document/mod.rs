use std::rc::Rc;
use crate::node::Node;
use crate::node::element_type::ElementType;
use crate::node::text::Text;
use crate::node_types::root::Root;

#[cfg(test)]
mod tests;

struct Document {
    root: Rc<dyn Node>,
}

impl Document {
    fn new() -> Document {
        let root = Self::create_element::<Root>((), Some(vec![]));

        Document {
            root
        }
    }

    pub fn create_element<T: ElementType>(
        attrs: T::Attributes,
        children: Option<Vec<Rc<dyn Node>>>,
    ) -> Rc<dyn Node> {
        T::create(Rc::new(attrs), children)
    }

    pub fn create_text(content: &str) -> Rc<dyn Node> {
        Text::new(String::from(content))
    }
}
