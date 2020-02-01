use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub value: String,
}

pub struct Text {
    props: Props,
}

pub enum Msg {}

impl Component for Text {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Text { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;

        true
    }

    fn view(&self) -> Html {
        html! {
            { self.props.value.as_str() }
        }
    }
}