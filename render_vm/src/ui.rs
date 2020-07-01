use std::rc::Rc;
use std::cell::RefCell;
use crate::instruction::Instruction;
use crate::html::{Node, NodeDescription, TypedNode, NodeId};
use crate::html::div::Div;

pub struct UI {
    id_counter: u64,
    root_element: Rc<RefCell<dyn Node>>,
    instructions: Rc<RefCell<Vec<Instruction>>>,
}

impl UI {
    pub fn new() -> Result<UI, String> {
        let instructions = Rc::new(RefCell::new(vec![]));
        let description = Div::new(())?;
        let root_element = TypedNode::new(0, None, description);

        Ok(UI {
            id_counter: 0,
            root_element,
            instructions,
        })
    }

    fn generate_element_id(&mut self) -> NodeId {
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

    pub fn create_element<T: NodeDescription>(
        &mut self,
        attrs: <T as NodeDescription>::Attributes,
    ) -> Result<Rc<RefCell<dyn Node>>, String> {
        let id = self.generate_element_id();
        let description = T::new(attrs)?;

        Ok(TypedNode::new(id, None, description))
    }
}
