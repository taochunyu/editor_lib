mod components;

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::components::editor::Editor;

pub struct App;

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Editor />
        }
    }
}
