type Description = Vec<String>;

pub enum Instruction {
    Create(Description),
    // [id, parent id, type, ...props]
    Append(Description),
    // [id, type, index, props, ...]
    // Update(Description),
}

impl Instruction {
    pub fn serialize(&self) -> String {
        match self {
            Instruction::Create(desc) => format!("[\"create\", {:?}]", desc),
            Instruction::Append(desc) => format!("[\"append\", {:?}]", desc),
            // Instruction::Update(desc) => format!("[\"update\", {:?}]", desc),
        }
    }
}
