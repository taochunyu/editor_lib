use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use render_vm::ui::UI;

pub struct View {
    ui: RefCell<UI>,
}

impl View {
    fn new() -> Rc<Self> {
        match UI::new() {
            Ok(ui) => {
                Rc::new(Self { ui: RefCell::new(ui) })
            },
            Err(msg) => panic!(msg),
        }
    }

    pub fn ui(&self) -> RefMut<UI>{
        self.ui.borrow_mut()
    }
}
