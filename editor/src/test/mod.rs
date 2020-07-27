pub mod tools {
    use std::rc::Rc;
    use crate::node::Node;
    use crate::node::utils::{create_text, create_element};
    use crate::node_types::paragraph::{Paragraph, ParagraphAttributes};
    use crate::node_types::root::{Root, RootAttributes};
    use crate::node::slice::Slice;
    use crate::Doc;

    pub fn create_doc() -> Doc {
        let hello = create_text("hello");
        let world = create_text("world");
        let paragraph_hello = create_element::<Paragraph>(ParagraphAttributes::new(), Some(vec![hello]));
        let paragraph_world = create_element::<Paragraph>(ParagraphAttributes::new(), Some(vec![world]));
        let root = create_element::<Root>(RootAttributes::new(), Some(vec![
            paragraph_hello,
            paragraph_world,
        ]));

        root
    }

    pub fn create_empty_slice() -> Slice {
        Slice::from(vec![])
    }

    pub fn create_slice_with_char() -> Slice {
        let char = create_text("c");

        Slice::from(char)
    }
}