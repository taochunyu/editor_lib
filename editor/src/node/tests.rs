use std::rc::Rc;
use crate::node::Node;
use crate::node::node_type::NodeType;

struct TextType;

impl NodeType for TextType {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

impl TextNodeType for TextType {}

struct MentionType;

impl NodeType for MentionType {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

struct ParagraphType;

impl NodeType for ParagraphType {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

struct RootType;

impl NodeType for RootType {
    type Attributes = ();

    fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}

#[test]
fn it_works() {
    let text_type = TextType::new();
    let mention_type = MentionType::new();
    let paragraph_type = ParagraphType::new();
    let root_type = RootType::new();

    let hello = text_type.create_node(Rc::new(()), "hello");
    // let world = Node::create_text_node("world");
    // let mention = mention_type.clone().create_node(Rc::new(()), None);
    // let paragraph = paragraph_type.clone().create_node(
    //     Rc::new(()),
    //     Some(vec![hello, mention, world]),
    // );
    // let root = root_type.create_node(Rc::new(()), Some(vec![paragraph]));
    //
    // let resolved = root.clone().resolve_offset(6);
    // println!("size: {}", root.size());
    // println!("content size: {}", root.content_size());
}
