use druid::{Widget, WidgetExt, Color, UnitPoint};
use druid::widget::{Flex, Align, Label, LabelText};
use crate::state::State;

pub fn build_sidebar() -> impl Widget<State> {
    let mut title = Label::new(LabelText::from("文档"));

    title.set_text_color(Color::from_rgba32_u32(0x585858ff));

    let mut content = Flex::column();

    content.add_spacer(20.0);

    content
        .add_child(Align::centered(title));

    let content = content
        .align_vertical(UnitPoint::TOP)
        .fix_width(300.0)
        .background(Color::from_rgba32_u32(0xfafafaff));

    let divider = Flex::column()
        .align_vertical(UnitPoint::TOP)
        .fix_width(1.0)
        .background(Color::from_rgba32_u32(0xe8e8e8ff));

    Flex::row()
        .with_child(content)
        .with_child(divider)
        .align_horizontal(UnitPoint::LEFT)
}
