use crate::instruction::Instruction;
use crate::element::{Element, ElementType, ElementId, Shared};
use std::rc::Rc;
use std::cell::RefCell;
use crate::element::root::{Root, RootProps};

pub struct UI {
    id_counter: u64,
    pub root_element: Shared<Element>,
    pub instructions: Shared<Vec<Instruction>>,
}

impl UI {
    pub fn new() -> Result<UI, String> {
        let instructions = Rc::new(RefCell::new(vec![]));
        let description = Root::new(RootProps { children: vec![] })?;
        let root_element = Rc::new(RefCell::new(Element {
            id: 0,
            description,
            parent_id: None,
            will_send_instructions: Rc::downgrade(&instructions),
        }));

        Ok(UI {
            id_counter: 0,
            root_element,
            instructions,
        })
    }

    fn generate_element_id(&mut self) -> ElementId {
        self.id_counter += 1;

        self.id_counter
    }

    pub fn flush(&mut self) -> Vec<String> {
        let result: Vec<String> = self.instructions.borrow().iter()
            .map(|instruction| instruction.serialize())
            .collect();

        self.instructions.borrow_mut().clear();

        result
    }

    pub fn create_element<ET: ElementType>(
        &mut self,
        props: <ET as ElementType>::Properties,
    ) -> Result<Shared<Element>, String> {
        let id = self.generate_element_id();
        let description = ET::new(props)?;
        let attached_ui = Rc::downgrade(&self.instructions);

        Ok(Rc::new(RefCell::new(Element {
            id,
            description,
            parent_id: None,
            will_send_instructions: attached_ui,
        })))
    }
}
