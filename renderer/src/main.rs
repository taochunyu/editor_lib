use std::rc::Rc;
use renderer::ui::UI;
use renderer::html::div::Div;
use renderer::html::paragraph::Paragraph;
use renderer::html::text::Text;
use renderer::html::operation::append_child;

fn main() {
    let mut ui = UI::new();

    let p1 = ui.create_element::<Paragraph>(());
    let t1 = ui.create_element::<Text>(String::from("1234"));

    append_child(p1, t1);

    println!("{}", ui.flush().join(" "));
}