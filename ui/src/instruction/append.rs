pub struct Append {
    pub(crate) description: Vec<String>,
}

impl Append {
    pub(crate) fn serialize(&self) -> String {
        format!("{:?}", self.description)
    }
}
