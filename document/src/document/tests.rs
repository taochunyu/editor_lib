use crate::document::Document;
use crate::node_types::paragraph::Paragraph;

#[test]
fn it_works() {
    let hello = Document::create_text("hello");
    let paragraph = Document::create_element::<Paragraph>(
        (),
        Some(vec![hello]),
    );

    println!("{}", paragraph.size());
}