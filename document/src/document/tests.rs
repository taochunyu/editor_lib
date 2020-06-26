use crate::document::Document;
use crate::node_types::paragraph::Paragraph;
use crate::node_types::root::Root;
use crate::node::slice::Slice;
use std::rc::Rc;

#[test]
fn it_works() {
    let hello = Document::create_text("hello");
    let paragraph = Document::create_element::<Paragraph>(
        (),
        Some(vec![hello]),
    );
    let doc = Document::create_element::<Root>((), Some(vec![paragraph]));
    let a = Document::create_text("a");
    let slice = Slice::new(0, 0, vec![a]);

    let result = doc.clone().replace(3, 3, slice);

    println!("{}", doc.find_path(3).unwrap().to_string());
}
