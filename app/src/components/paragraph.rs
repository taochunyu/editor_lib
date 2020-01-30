use yew::prelude::*;
use yew::html;
use crate::components::text::Text;

#[derive(Clone, Properties)]
pub struct Props {
    children: Children,
}

pub struct Paragraph {
   props: Props,
}

pub enum Msg {}

impl Component for Paragraph {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Paragraph {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <p class="vf-paragraph">
                { self.props.children.render() }
            </p>
        }
    }
}