use yew::prelude::*;
use model::node::node::Node;
use model::node::content::Content;
use crate::components::{
    doc,
    paragraph,
    text,
};
use std::rc::Rc;
use yew::virtual_dom::VNode;
use std::ops::Deref;
use model::document::document::Document;

pub struct Editor;

pub enum Msg {}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Editor
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let doc = Document::new();

        html! {
            <div class="vf-editor">
                { render(doc.root()) }
            </div>
        }
    }
}

fn render(node: &Rc<Node>) -> Html {
    match node.node_type().name().as_str() {
        "text" => match node.content().deref() {
            Content::Text(text) => {
                let props = text::Props {
                    value: String::from(text)
                };

                html! {
                    <text::Text with props />
                }
            },
            _ => html! {},
        },
        "doc" => match node.content().deref() {
            Content::Elements(fragment) => {
                html! {
                    <doc::Doc>
                        { for fragment.content().iter().map(render) }
                    </doc::Doc>
                }
            },
            _ => html! {},
        },
        "paragraph" => match node.content().deref() {
            Content::Elements(fragment) => {
                html! {
                    <paragraph::Paragraph>
                        { for fragment.content().iter().map(render) }
                    </paragraph::Paragraph>
                }
            },
            _ => html! {},
        },
        _ => html! {},
    }
}