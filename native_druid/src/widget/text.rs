use druid::piet::{
    FontFamily, PietText, PietTextLayout, Text, TextLayout, TextLayoutBuilder, UnitPoint,
};
use druid::{
    theme, BoxConstraints, Color, Data, Env, Event, EventCtx, KeyOrValue, LayoutCtx, LifeCycle,
    LifeCycleCtx, LocalizedString, PaintCtx, Point, RenderContext, Size, UpdateCtx, Widget,
};

pub struct BasicText {
    text: String,
    color: Color,
    size: f64,
    font: &'static str,
}

impl BasicText {
    pub fn new(text: String) -> Self {
        Self {
            text,
            color: Color::BLACK,
            size: 18.0,
            font: FontFamily::SYSTEM_UI.name(),
        }
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.color = color;

        self
    }

    pub fn with_text_size(mut self, size: f64) -> Self {
        self.size = size;

        self
    }

    pub fn with_font(mut self, font: &'static str) -> Self {
        self.font = font;

        self
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_text_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn set_font(&mut self, font: &'static str) {
        self.font = font;
    }

    fn get_layout(&mut self, t: &mut PietText, _env: &Env) -> PietTextLayout {
        let font = t.font_family(self.font).unwrap_or(FontFamily::SYSTEM_UI);

        t.new_text_layout(self.text.as_str())
            .font(font, self.size)
            .text_color(self.color.clone())
            .build()
            .unwrap()
    }
}

impl<T: Data> Widget<T> for BasicText {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut T, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        if !old_data.same(data) {
            ctx.request_layout();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        bc.debug_check("PureText");

        let text_layout = self.get_layout(&mut ctx.text(), env);
        let text_size = text_layout.size();

        bc.constrain(text_size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        let text_layout = self.get_layout(&mut ctx.text(), env);
        let origin = Point::new(0.0, 0.0);

        ctx.draw_text(&text_layout, origin);
    }
}
