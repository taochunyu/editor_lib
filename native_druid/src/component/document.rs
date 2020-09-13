use druid::{Widget, WidgetExt, Color, UnitPoint, Data, EventCtx, Event, Env, LifeCycleCtx, LifeCycle, UpdateCtx, LayoutCtx, BoxConstraints, Size, PaintCtx, Point, RenderContext, WidgetPod, Rect};
use druid::piet::{PietText};
use druid::widget::{Flex, Label, LabelText, Align, FlexParams, CrossAxisAlignment};
use crate::state::State;
use crate::service::block_layout::{Node, Layout, LayoutContext};
use crate::widget::text::BasicText;

pub struct Document<T> {
    nodes: Vec<Node>,
    layout: Layout,
    layout_context: Option<LayoutContext>,
    children: Vec<WidgetPod<T, Box<dyn Widget<T>>>>,
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> Document<T> {
    pub fn new() -> Self {
        let text = Node::Text(String::from("hello world!"));
        let label = Label::new(LabelText::from("Hello World!")).with_text_color(Color::BLACK);

        Self {
            nodes: vec![text],
            layout: Layout::new(),
            layout_context: None,
            children: vec![],
            inner: WidgetPod::new(Box::new(label)),
        }
    }

    fn get_layout(&mut self, piet_text: &mut PietText) -> Size {
        let ctx = self.layout.layout(piet_text, self.nodes.clone(), true);
        let size = Size::new(ctx.width(), ctx.height());

        self.layout_context = Some(ctx);

        size
    }

    fn create_children_widget(&mut self) {
        if let Some(ctx) = &self.layout_context {
            let mut result = vec![];

            for row in ctx.rows() {
                for (_, block) in &row.blocks {
                    if let Some(text) = block.text() {
                        let label = Label::new(LabelText::from(text));

                        result.push(WidgetPod::<T, Box<dyn Widget<T>>>::new(Box::new(label)));
                    }
                }
            }

            self.children = result;
        }
    }
}

impl<T: Data> Widget<T> for Document<T> {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Document");

        let size = self.inner.layout(ctx, &bc, data, env);
        // let origin = Point::new(0.0, 0.0);
        // self.inner
        //     .set_layout_rect(ctx, data, env, Rect::from_origin_size(origin, size));

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        // self.create_children_widget();
        //
        // println!("{}", self.children.len());
        //
        // for child in &mut self.children {
        //     child.paint(ctx, data, env)
        // }

        self.inner.paint(ctx, data, env)
    }
}

pub fn build_document() -> impl Widget<State> {
    Flex::column()
}
