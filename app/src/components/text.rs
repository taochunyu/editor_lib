use yew::prelude::*;
use yew::virtual_dom::vnode::VNode;
use yew::virtual_dom::VText;

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

    fn view(&self) -> Html {
        VNode::VText(VText::new(self.props.value.clone() ))
    }
}