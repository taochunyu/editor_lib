use crate::element::{ElementType, ElementDescription, ElementResult, Element, Shared};

pub struct RectangularProps {
    pub width: u32,
    pub height: u32,
}

pub struct Rectangular {
    width: u32,
    height: u32,
    children: Vec<Shared<Element>>,
}

impl ElementType for Rectangular {
    type Properties = RectangularProps;

    fn name() -> String {
        String::from("rectangular")
    }

    fn new(props: RectangularProps) -> ElementResult<ElementDescription> {
        ElementResult::Ok(ElementDescription::Rectangular(Rectangular {
            width: props.width,
            height: props.height,
            children: vec![],
        }))
    }

    fn to_instruction_description(&self) -> Vec<String> {
        vec![Self::name(), format!("{}", self.width), format!("{}", self.height)]
    }

    fn descendants(&self) -> Vec<Shared<Element>> {
        self.children.clone()
    }

    fn append_child(&mut self, element: Shared<Element>) -> ElementResult<()> {
        self.children.push(element);

        Ok(())
    }
}
