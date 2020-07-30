use crate::html::selection::Selection;

enum InputType {
    InsertText(String),
    InsertCompositionText(String),
    InsertParagraph,
    DeleteContentBackward,
}

pub struct InputEvent {
    selection: Selection,
    is_composing: bool,
    input_type: InputType,
}
