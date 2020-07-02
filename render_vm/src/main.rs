use std::rc::Rc;
use render_vm::ui::UI;
use render_vm::html::div::Div;
use render_vm::html::paragraph::Paragraph;
use render_vm::html::text::Text;
use render_vm::html::operation::append_child;

fn main() {
    let mut ui = UI::new();

    let p1 = ui.create_element::<Paragraph>(());
    let t1 = ui.create_element::<Text>(String::from("1234"));

    append_child(p1, t1);

    println!("{}", ui.flush().join(" "));
}