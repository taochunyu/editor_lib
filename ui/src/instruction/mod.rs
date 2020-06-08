pub mod append;

use crate::instruction::append::Append;

pub enum Instruction {
    Append(Append),
}

impl Instruction {
    pub fn serialize(&self) -> String {
        match self {
            Instruction::Append(append) => format!("[\"append\", {}]", append.serialize()),
        }
    }
}
