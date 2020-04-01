use crate::components::{doc, paragraph, text};
use editor::document::document::Document;
use editor::node::content::Content;
use editor::node::node::Node;
use editor::slice::slice::Slice;
use std::ops::Deref;
use std::rc::Rc;
use std::time::SystemTime;
use stdweb::traits::IHtmlElement;
use stdweb::web::html_element::InputElement;
use yew::prelude::*;
use yew::services::ConsoleService;
use yew::virtual_dom::VNode;

pub struct Editor {
    doc: Rc<Document>,
    link: ComponentLink<Self>,
    input_ref: NodeRef,
}

pub enum Msg {
    Keydown,
    Click,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            doc: Document::new(),
            link,
            input_ref: NodeRef::default(),
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(input) = self.input_ref.try_into::<InputElement>() {
            input.focus();
        }

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut console = ConsoleService::new();

        match msg {
            Msg::Keydown => {
                console.time();

                self.doc = Rc::new(self.doc.replace(800, 8000, Slice::text("t")).unwrap());

                console.time_end();
            }
            Msg::Click => {
                if let Some(input) = self.input_ref.try_into::<InputElement>() {
                    input.focus();
                }
            }
        }

        true
    }

    fn view(&self) -> Html {
        let handle_click = self.link.callback(|_| Msg::Click);
        let handle_key_down = self.link.callback(|_| Msg::Keydown);

        html! {
            <div class="vf-editor" onclick=handle_click>
                <input
                    type="text"
                    ref=self.input_ref.clone()
                    class="vf-input"
                    onkeydown=handle_key_down
                />
                { render(self.doc.root()) }
            </div>
        }
    }
}

fn render(node: &Rc<Node>) -> Html {
    match node.node_type().name().as_str() {
        "text" => match node.content().deref() {
            Content::Text(text) => {
                let props = text::Props {
                    value: String::from(text),
                };

                return html! {
                    <text::Text with props />
                };
            }
            _ => html! {},
        },
        "doc" => match node.content().deref() {
            Content::Elements(fragment) => {
                return html! {
                    <doc::Doc>
                        { for fragment.content().iter().map(render) }
                    </doc::Doc>
                }
            }
            _ => html! {},
        },
        "paragraph" => match node.content().deref() {
            Content::Elements(fragment) => {
                return html! {
                    <paragraph::Paragraph>
                        { for fragment.content().iter().map(render) }
                    </paragraph::Paragraph>
                }
            }
            _ => html! {},
        },
        _ => html! {},
    }
}
