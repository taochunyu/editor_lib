use crate::element::{ElementType, ElementDescription, ElementResult, Element, Shared};

pub struct RootProps {
    pub(crate) children: Vec<Shared<Element>>,
}

pub struct Root {
    children: Vec<Shared<Element>>,
}

impl ElementType for Root {
    type Properties = RootProps;

    fn name() -> String {
        String::from("root")
    }

    fn new(props: RootProps) -> Result<ElementDescription, String> {
        ElementResult::Ok(ElementDescription::Root(Root {
            children: props.children,
        }))
    }

    fn to_instruction_description(&self) -> Vec<String> {
        vec![Self::name()]
    }

    fn descendants(&self) -> Vec<Shared<Element>> {
        self.children.clone()
    }


    fn append_child(&mut self, element: Shared<Element>) -> Result<(), String> {
        self.children.push(element);

        Ok(())
    }
}
