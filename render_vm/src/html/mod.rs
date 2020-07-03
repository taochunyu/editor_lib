pub mod operation;
pub mod text;
pub mod div;
pub mod paragraph;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::instruction::Instruction;

pub trait Node {
    fn id(&self) -> NodeId;
    fn description(&self) -> Vec<String>;
    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>>;
    fn set_parent_id(&mut self, parent_id: Option<NodeId>);
    fn append_child(&mut self, child: Rc<RefCell<dyn Node>>);
}

pub trait NodeDescription: Sized + 'static {
    type Attributes;

    fn name() -> &'static str;
    fn new(attrs: Self::Attributes) -> Self;
    fn to_instruction(&self) -> Vec<String>;
    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        vec![]
    }
    fn append_child(&mut self, _child: Rc<RefCell<dyn Node>>) {
        unimplemented!()
    }
    fn insert_before(self, _base: NodeId, _node: Rc<RefCell<dyn Node>>) -> Result<(), String> {
        unimplemented!()
    }
    fn remove_child(self, _child_id: NodeId) -> Result<(), String> {
        unimplemented!()
    }
}

pub type NodeId = u64;

pub struct TypedNode<T: NodeDescription> {
    id: NodeId,
    parent_id: Option<NodeId>,
    description: T,
    instructions: Weak<RefCell<Vec<Instruction>>>,
}

impl<T: NodeDescription> Node for TypedNode<T> {
    fn id(&self) -> NodeId {
        self.id
    }

    fn description(&self) -> Vec<String> {
        let parent_id = match self.parent_id {
            Some(id) => format!("{}", id),
            None => String::new(),
        };

        [vec![format!("{}", self.id), parent_id], self.description.to_instruction()].concat()
    }

    fn children(&self) -> Vec<Rc<RefCell<dyn Node>>> {
        self.description.children()
    }

    fn set_parent_id(&mut self, parent_id: Option<NodeId>) {
        self.parent_id = parent_id;
    }

    fn append_child(&mut self, child: Rc<RefCell<dyn Node>>) {
        self.description.append_child(child.clone());

        let mut child_mut = child.borrow_mut();

        child_mut.set_parent_id(Some(self.id));

        if let Some(instructions) = self.instructions.upgrade() {
            instructions.borrow_mut().push(Instruction::Append(child_mut.description()));
        };
    }
}

impl<T: NodeDescription> TypedNode<T> {
    pub(crate) fn new(id: NodeId, parent_id: Option<NodeId>, description: T, instructions: Weak<RefCell<Vec<Instruction>>>) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(Self {
            id,
            parent_id,
            description,
            instructions,
        }))
    }
}
