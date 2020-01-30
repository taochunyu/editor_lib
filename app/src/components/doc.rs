use yew::prelude::*;
use crate::components::text::Text;

#[derive(Clone, Properties)]
pub struct Props {
    children: Children,
}

pub struct Doc {
    props: Props,
}

pub enum Msg {}

impl Component for Doc {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Doc {
            props
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.props.children.render() }
            </>
        }
    }
}