use web_sys::{Document, Node, window};
use renderer::host::Host;

pub struct Browser {
    document: Document,
    root_instance: Node,
}

impl Host for Browser {
    type Instance = Node;

    fn root_instance(&self) -> Self::Instance {
        self.root_instance
    }

    fn create_instance(&self, name: &str) -> Self::Instance {
        self.document.create_element(name).unwrap().into()
    }

    fn create_text_instance(&self, content: &str) -> Self::Instance {
        self.document.create_text_node(content).into()
    }

    fn append_child(parent: &Self::Instance, child: Self::Instance) {
        parent.append_child(&child);
    }
}

impl Browser {
    pub fn new(root_id: &str) -> Self {
        let document = window().unwrap().document().unwrap();
        let root_instance: Node = document.query_selector(root_id).unwrap().unwrap().into();

        Self { document, root_instance }
    }
}
