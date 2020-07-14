pub mod tools {
    use std::rc::Rc;
    use crate::node::Node;
    use crate::node::utils::{create_text, create_element};
    use crate::node_types::paragraph::Paragraph;
    use crate::node_types::root::Root;
    use crate::node::slice::Slice;

    pub fn create_root() -> Rc<dyn Node> {
        let hello = create_text("hello");
        let world = create_text("world");
        let paragraph_hello = create_element::<Paragraph>((), Some(vec![hello]));
        let paragraph_world = create_element::<Paragraph>((), Some(vec![world]));
        let root = create_element::<Root>((), Some(vec![
            paragraph_hello,
            paragraph_world,
        ]));

        root
    }

    pub fn create_empty_slice() -> Slice {
        Slice::new(0, 0, vec![])
    }

    pub fn create_slice_with_char() -> Slice {
        let char = create_text("c");

        Slice::new(0, 0, vec![char])
    }
}