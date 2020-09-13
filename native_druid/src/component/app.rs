use druid::{Widget, WidgetExt, Color, Data, Lens, UnitPoint};
use druid::widget::{Flex, FlexParams, CrossAxisAlignment, List};
use crate::component::document::build_document;
use crate::state::State;
use crate::component::sidebar::build_sidebar;
use crate::component::doc::Doc;

pub fn build_app() -> impl Widget<State> {
    let sidebar = build_sidebar();
    let document = Doc::<State>::new();

    let wrapper = Flex::column()
        .with_child(document)
        .align_vertical(UnitPoint::TOP);

    Flex::row()
        .with_child(sidebar)
        .with_child(wrapper)
        .align_horizontal(UnitPoint::LEFT)
        .background(Color::WHITE)
}
