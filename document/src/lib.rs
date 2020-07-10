use std::rc::Rc;
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use editor::node::utils::{create_text, create_element};
use editor::node_types::paragraph::Paragraph;
use editor::node_types::root::Root;
use editor::view::View;

pub struct Document {
    renderer: Rc<Renderer>,
}

impl Document {
    pub fn new(renderer: Renderer) -> Self {
        Self {
            renderer: Rc::new(renderer),
        }
    }

    pub fn trigger_test_doc(&self) {
        let div = self.renderer.create_element::<HTMLDivElement>();

        self.renderer.root().append_child(&div.clone().into());

        let mut content = vec![];

        for _ in 0..1 {
            let hello = create_text("hello, world");
            let paragraph = create_element::<Paragraph>(
                (),
                Some(vec![hello]),
            );

            content.push(paragraph);
        }

        let doc = create_element::<Root>((), Some(content));
        let view = View::new(self.renderer.clone(), div, doc);

        view.init();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
