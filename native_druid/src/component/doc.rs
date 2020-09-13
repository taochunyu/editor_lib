use druid::{WidgetPod, Widget, LifeCycle, EventCtx, PaintCtx, LifeCycleCtx, BoxConstraints, Size, LayoutCtx, Event, Env, UpdateCtx, Data, WidgetExt, Point, Rect, RenderContext, UnitPoint};
use druid::piet::{PietText};
use crate::service::block_layout::{Layout, Node};
use druid::widget::{Flex, Label, LabelText, Align, MainAxisAlignment};
use crate::widget::text::BasicText;

pub struct Doc<T> {
    text: String,
    service: Layout,
    inner: Flex<T>,
}

impl<T: Data> Doc<T> {
    pub fn new() -> Self {
        let text = String::from("hello world! hello world! hello world! hello world! hello world! hello world! hello world! hello world! hello world! hello world! hello world! ");
        let service = Layout::new();
        let inner = Flex::column();

        Self {
            text,
            service,
            inner,
        }
    }

    fn handle_added(&mut self, piet_text: &mut PietText) {
        let nodes = vec![Node::Text(self.text.clone())];
        let ctx = self.service.layout(piet_text, nodes, false);

        for line in ctx.rows() {
            let mut row = Flex::row();

            for (_, block) in &line.blocks {
                let size = block.size();
                let text = block.text().unwrap_or(String::new());
                let label = BasicText::new(text);

                row.add_child(label.fix_size(size.width, size.height));
            }

            self.inner.add_child(row.align_horizontal(UnitPoint::CENTER).padding(20.0));
        }

    }
}

impl<T: Data> Widget<T> for Doc<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => {
                self.handle_added(&mut ctx.text());
            },
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, &bc, data, env);
        // let origin = Point::new(0.0, 0.0);
        // self.inner
        //     .set_layout_rect(ctx, data, env, Rect::from_origin_size(origin, size));

        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}
