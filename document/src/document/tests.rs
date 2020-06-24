use crate::document::Document;
use crate::node_types::paragraph::Paragraph;

#[test]
fn it_works() {
    Document::create_element::<Paragraph>((), None);
}