use crate::element::rectangular::Rectangular;
use crate::element::text::Text;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use crate::instruction::Instruction;
use crate::element::root::Root;

pub mod rectangular;
pub mod root;
pub mod text;

pub type ElementId = u64;

type ElementResult<T> = Result<T, String>;

pub type Shared<T> = Rc<RefCell<T>>;

pub trait ElementType where Self: Sized {
    type Properties;

    fn name() -> String;
    fn new(props: Self::Properties) -> ElementResult<ElementDescription>;
    fn to_instruction_description(&self) -> Vec<String>;
    fn descendants(&self) -> Vec<Shared<Element>> {
        vec![]
    }
    fn append_child(&mut self, _element: Shared<Element>) -> Result<(), String> {
        unimplemented!()
    }
    fn insert_before(self, _base_element_id: ElementId, _element: Shared<Element>) -> Result<ElementDescription, String> {
        unimplemented!()
    }
    fn remove_child(self, _element_id: ElementId) -> Result<ElementDescription, String> {
        unimplemented!()
    }
}

pub enum ElementDescription {
    Rectangular(Rectangular),
    Root(Root),
    Text(Text),
}

impl ElementDescription {
    fn to_instruction_description(&self) -> Vec<String> {
        match self {
            ElementDescription::Rectangular(rect) => rect.to_instruction_description(),
            ElementDescription::Root(root) => root.to_instruction_description(),
            ElementDescription::Text(text) => text.to_instruction_description(),
        }
    }
    fn descendants(&self) -> Vec<Shared<Element>> {
        match self {
            ElementDescription::Rectangular(rect) => rect.descendants(),
            ElementDescription::Root(root) => root.descendants(),
            ElementDescription::Text(text) => text.descendants(),
        }
    }
    fn append_child(&mut self, element: Shared<Element>) -> Result<&mut ElementDescription, String> {
        match self {
            ElementDescription::Rectangular(rect) => rect.append_child(element),
            ElementDescription::Root(root) => root.append_child(element),
            ElementDescription::Text(text) => text.append_child(element),
        }?;

        Ok(self)
    }
}

pub struct Element {
    pub(crate) id: ElementId,
    pub(crate) description: ElementDescription,
    pub(crate) parent_id: Option<ElementId>,
    pub(crate) will_send_instructions: Weak<RefCell<Vec<Instruction>>>,
}

impl Element {
    fn to_instruction(&self) -> Vec<String> {
        let parent_id = if let Some(p_id) = self.parent_id {
            format!("{}", p_id)
        } else {
            String::new()
        };

        [
            vec![format!("{}", self.id), parent_id],
            self.description.to_instruction_description(),
        ].concat()
    }

    fn elements(self_: &Shared<Self>) -> Vec<Shared<Element>> {
        let descendants = self_.borrow().description.descendants();

        [vec![self_.clone()], descendants].concat()
    }

    fn set_parent_id(&mut self, parent_id: ElementId) {
        self.parent_id = Some(parent_id);
    }

    pub fn append_child(&mut self, element: Shared<Element>) -> Result<&mut Element, String> {
        element.borrow_mut().set_parent_id(self.id);

        if let Some(will_send) = self.will_send_instructions.upgrade() {
            let mut instructions: Vec<Instruction> = Self::elements(&element).iter()
                .map(|elm| elm.borrow().to_instruction())
                .map(|description| Instruction::Append(description))
                .collect();

            will_send.borrow_mut().append(&mut instructions);
        }

        self.description.append_child(element)?;

        Ok(self)
    }
}
