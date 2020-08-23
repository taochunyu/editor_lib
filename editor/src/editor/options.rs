use crate::editor::Editor;

pub struct Options {
    editable: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            editable: true,
        }
    }
}

impl Editor {
    // pub fn editable(&self) -> bool {
    //     self.options.editable
    // }
    //
    // pub fn set_editable(&mut self, editable: bool) -> &mut Self {
    //     self.options.editable = editable;
    //
    //     self
    // }
}
