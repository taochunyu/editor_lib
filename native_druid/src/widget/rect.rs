use druid::{
    theme, BoxConstraints, Color, Data, Env, Event, EventCtx, KeyOrValue, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, RenderContext, Size, UpdateCtx, Widget,
};

pub struct BasicRect {
    size: Size,
}

impl BasicRect {
    pub fn new(size: Size) -> Self {
        Self { size }
    }

    pub fn set_size(&mut self, size: Size) {
        self.size = size
    }

    pub fn size(&self) -> Size {
        self.size
    }
}

impl<T: Data> Widget<T> for BasicRect {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {}

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        bc.debug_check("BasicRect");

        self.size()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {}
}
