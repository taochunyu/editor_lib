use std::rc::Rc;
use render_vm::ui::UI;
use render_vm::element::rectangular::{RectangularProps, Rectangular};

fn main() -> Result<(), String> {
    let mut ui = UI::new()?;
    let rect_props_1 = RectangularProps { width: 200, height: 100 };
    let rect_1 = ui.create_element::<Rectangular>(rect_props_1)?;

    let rect_props_2 = RectangularProps { width: 200, height: 100 };
    let rect_2 = ui.create_element::<Rectangular>(rect_props_2)?;

    ui.root_element.borrow_mut().append_child(Rc::clone(&rect_1))?;

    rect_1.borrow_mut().append_child(rect_2)?;

    for instruction_str in ui.flush().iter() {
        println!("{}", instruction_str);
    }

    Ok(())
}