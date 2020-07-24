use std::rc::Rc;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use editor::node::utils::{create_text, create_element};
use editor::node_types::paragraph::Paragraph;
use editor::node_types::root::Root;
use editor::view::View;

pub struct Document {
    view: View,
}

impl Document {
    pub fn new(renderer: Rc<Renderer>) -> Self {
        let div = renderer.create_element::<HTMLDivElement>();

        renderer.root().append_child(&div.clone().into());

        let mut content = vec![];

        for _ in 0..10 {
            let hello = create_text("hello, world");
            let paragraph = create_element::<Paragraph>(
                (),
                Some(vec![hello]),
            );

            content.push(paragraph);
        }

        let doc = create_element::<Root>((), Some(content));

        Self {
            view: View::new(renderer, div, doc)
        }
    }

    pub fn trigger_test(&mut self) {

    }
}

#[cfg(test)]
mod test {
    use renderer::host::test_host::TestHost;
    use std::rc::Rc;
    use renderer::Renderer;
    use editor::view::View;
    use crate::Document;

    #[test]
    fn doc_init_works() {
        let test_host = TestHost::new();
        let renderer = Rc::new(Renderer::new(test_host));

        Document::new(renderer);
    }
}
