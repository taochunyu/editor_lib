mod layout;

use druid::widget::{Button, Flex, Label, TextBox, LabelText};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Env};
use druid::piet::{Text, PietText, FontBuilder, TextLayoutBuilder, TextLayout};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)

}

fn ui_builder() -> impl Widget<u32> {
    let text = LabelText::from("123");
    let mut label = Label::new(text);

    label.set_text_size(36.0);


    Flex::column().with_child(label)
}
// fn timestamp_ms() -> i64 {
//     let start = SystemTime::now();
//     let since_the_epoch = start
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//
//     since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64
// }
//
// fn main() {
//     let mut w = 0.0;
//     let mut piet_text = PietText::new();
//     let font = piet_text.new_font_by_name("Arial", 19.0).build().unwrap();
//     let a = timestamp_ms();
//     for _ in 0..1 {
//         w += piet_text.new_text_layout(&font, "123456789", std::f64::INFINITY).build().unwrap().width();
//     }
//     let b = timestamp_ms();
//     println!("{}, {}", w, b - a);
// }