use web_sys::{Document, Node, window};
use renderer::host::Host;

pub struct Browser {
    document: Document,
    root_id: &'static str,
}

impl Host for Browser {
    type Instance = Node;

    fn root_instance(&self) -> Self::Instance {
        let document = window().unwrap().document().unwrap();

        document.query_selector(self.root_id).unwrap().unwrap().into()
    }

    fn create_instance(&self, name: &str) -> Self::Instance {
        self.document.create_element(name).unwrap().into()
    }

    fn create_text_instance(&self, content: &str) -> Self::Instance {
        self.document.create_text_node(content).into()
    }

    fn append_child(parent: &Self::Instance, child: Self::Instance) {
        web_sys::console::log_1(&"1234".into());
        parent.append_child(&child);
    }
}

impl Browser {
    pub fn new(root_id: &'static str) -> Self {
        let document = window().unwrap().document().unwrap();

        Self { document, root_id }
    }
}
