use crate::node::node::Node;
use std::rc::Rc;

pub struct ResolvedPosition {
    position: usize,
    path: Vec<(Rc<Node>, usize, usize)>,
    parent_offset: usize,
    depth: usize,
}

impl ResolvedPosition {
    pub fn position(&self) -> usize {
        self.position
    }
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn resolve(base: Rc<Node>, position: usize) -> Result<ResolvedPosition, String> {
        if position > base.content_size() {
            return Err(format!("Position {} out of range", position));
        }

        let mut path: Vec<(Rc<Node>, usize, usize)> = vec![];
        let mut start: usize = 0;
        let mut parent_offset: usize = position;
        let mut cursor: Rc<Node> = base;

        loop {
            let (index, offset) = cursor.node_content().find_index(parent_offset, false)?;
            let rem = parent_offset - offset;

            path.push((Rc::clone(&cursor), index, start + offset));

            if rem == 0 {
                break;
            }

            cursor = cursor.child(index)?;

            if cursor.is_text() {
                break;
            }

            parent_offset = rem - 1;
            start += offset + 1;
        }

        Ok(ResolvedPosition {
            depth: path.len() - 1,
            position,
            path,
            parent_offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::node::node::Node;
    use crate::schema::node_type::NodeType;
    use crate::node::node_content::NodeContent;
    use std::rc::Rc;
    use crate::position::resolved_position::ResolvedPosition;

    fn mock_text_node(content: &str) -> Node {
        let node_type = NodeType::new(String::from("text"), String::from(""));
        let node_content = NodeContent::Text(String::from(content));

        node_type.create_node(Rc::new(node_content))
    }

    fn mock_leaf_node(name: &str) -> Node {
        let node_type = NodeType::new(String::from(name), String::from(""));

        node_type.create_node(Rc::new(NodeContent::None))
    }

    fn mock_container_node(name: &str, content: NodeContent) -> Node {
        let node_type = NodeType::new(String::from(name), String::from("123131"));

        node_type.create_node(Rc::new(content))
    }

    fn mock_data() -> Node {
        let apple = mock_text_node("apple");
        let hello = mock_text_node("hello");
        let world = mock_text_node("world");
        let image = mock_leaf_node("image");
        assert_eq!(hello.size(), 5);
        assert_eq!(image.size(), 1);

        let paragraph_1 = mock_container_node("paragraph", NodeContent::from(Rc::new(apple)));
        let paragraph_2 = mock_container_node(
            "paragraph",
            NodeContent::from(vec![Rc::new(hello), Rc::new(image), Rc::new(world)])
        );

        assert_eq!(paragraph_1.size(), 7);
        assert_eq!(paragraph_2.size(), 13);


        mock_container_node("doc", NodeContent::from(vec![Rc::new(paragraph_1), Rc::new(paragraph_2)]))
    }

    fn check_resolve_result(base: &Rc<Node>, position: usize, depth: usize, parent_offset: usize) {
        match ResolvedPosition::resolve(Rc::clone(base), position) {
            Ok(resolved) => {
                assert_eq!(resolved.depth, depth);
                assert_eq!(resolved.parent_offset, parent_offset)
            },
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn it_works() {
        let doc = Rc::new(mock_data());

        assert_eq!(doc.size(), 22);

        check_resolve_result(&doc, 0, 1, 0);
        check_resolve_result(&doc, 3, 2, 2);
        check_resolve_result(&doc, 7, 1, 7);
        check_resolve_result(&doc, 8, 2, 0);
    }
}
