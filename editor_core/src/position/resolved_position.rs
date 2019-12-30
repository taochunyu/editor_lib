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
    pub fn index(&self, depth: usize) -> Result<usize, String> {
        match self.path.get(depth) {
            Some(node) => Ok(node.1),
            None => Err(format!("{}", depth)),
        }
    }
    pub fn node(&self, depth: usize) -> Result<Rc<Node>, String> {
        match self.path.get(depth) {
            Some(node) => Ok(Rc::clone(&node.0)),
            None => Err(format!("{}", depth)),
        }
    }
    pub fn node_after(&self) -> Result<Option<Rc<Node>>, String> {
        let parent = self.parent()?;
        let index = self.index(self.depth)?;

        if index == parent.content().count() {
            Ok(None)
        } else {
            let text_offset = self.text_offset()?;
            let node = parent.get(index)?;

            if text_offset == 0 {
                Ok(Some(Rc::clone(node)))
            } else {
                let n = node.clone().cut(text_offset, node.size())?;

                Ok(Some(n))
            }
        }
    }
    pub fn parent(&self) -> Result<Rc<Node>, String> {
        self.node(self.depth)
    }
    pub fn text_offset(&self) -> Result<usize, String> {
        match self.path.last() {
            Some(n) => Ok(self.position - n.2),
            None => Err(format!("ResolvedPosition: path is empty"))
        }
    }

    pub fn resolve(base: &Rc<Node>, position: usize) -> Result<ResolvedPosition, String> {
        if position > base.content().size() {
            return Err(format!("Position {} out of range", position));
        }

        let mut path: Vec<(Rc<Node>, usize, usize)> = vec![];
        let mut start: usize = 0;
        let mut parent_offset: usize = position;
        let mut cursor: &Rc<Node> = base;

        loop {
            let (index, offset) = cursor.content().find_index(parent_offset, false)?;
            let rem = parent_offset - offset;

            path.push((Rc::clone(&cursor), index, start + offset));

            if rem == 0 {
                break;
            }

            cursor = cursor.get(index)?;

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
    use crate::node::content::Content;
    use std::rc::Rc;
    use crate::position::resolved_position::ResolvedPosition;

    fn mock_text_node(content: &str) -> Node {
        let node_type = NodeType::new(String::from("text"), String::from(""));
        let node_content = Content::Text(String::from(content));

        node_type.create_node(Rc::new(node_content))
    }

    fn mock_leaf_node(name: &str) -> Node {
        let node_type = NodeType::new(String::from(name), String::from(""));

        node_type.create_node(Rc::new(Content::None))
    }

    fn mock_container_node(name: &str, content: Content) -> Node {
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

        let paragraph_1 = mock_container_node("paragraph", Content::from(Rc::new(apple)));
        let paragraph_2 = mock_container_node(
            "paragraph",
            Content::from(vec![Rc::new(hello), Rc::new(image), Rc::new(world)])
        );

        assert_eq!(paragraph_1.size(), 7);
        assert_eq!(paragraph_2.size(), 13);


        mock_container_node("doc", Content::from(vec![Rc::new(paragraph_1), Rc::new(paragraph_2)]))
    }

    fn check_resolve_result(base: &Rc<Node>, position: usize, depth: usize, parent_offset: usize) {
        match ResolvedPosition::resolve(&Rc::clone(base), position) {
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

        check_resolve_result(&doc, 0, 0, 0);
        check_resolve_result(&doc, 3, 1, 2);
        check_resolve_result(&doc, 7, 0, 7);
        check_resolve_result(&doc, 8, 1, 0);
    }
}
