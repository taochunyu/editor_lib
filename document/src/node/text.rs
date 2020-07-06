use std::rc::Rc;
use std::any::Any;
use renderer::html::text;
use crate::node::fragment::Fragment;
use crate::node::element_type::{OuterDOM, ContentDOM};
use crate::view::View;
use crate::node::Node;


pub struct Text {
    marks: Vec<u64>,
    content: String,
}

impl Node for Text {
    fn size(&self) -> usize {
        self.content_size()
    }

    fn content_size(&self) -> usize {
        self.content.len()
    }

    fn child_count(&self) -> usize {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cut_node(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        match self.content.get(from..to) {
            Some(sub) => {
                Ok(Rc::new(Self {
                    marks: vec![],
                    content: String::from(sub),
                }))
            },
            None => Err(format!("rang {}..{} outside of text", from, to))
        }
    }

    fn index(&self, _: usize) -> Result<usize, String> {
        Err(format!("Cannot find offset index on text node."))
    }

    fn get_child(&self, _: usize) -> Result<Rc<dyn Node>, String> {
        Err(format!("Cannot get child on text node."))
    }

    fn children(&self) -> Option<Rc<Fragment>> {
        None
    }

    fn replace_children(&self, _: Option<Rc<Fragment>>) -> Result<Rc<dyn Node>, String> {
        Err(format!("Text node cannot replace children."))
    }

    fn to_html_string(&self) -> String {
        match self.marks.len() {
            0 => self.content.clone(),
            _ => format!("<span>{}</span>", self.content)
        }
    }

    fn render(self: Rc<Self>, view: Rc<View>) -> (OuterDOM, ContentDOM) {
        let text = view.clone().ui().create_element::<text::Text>(self.content.clone());

        (text, None)
    }
}

impl Text {
    pub(crate) fn new(content: String) -> Rc<dyn Node> {
        Rc::new(Self {
            marks: vec![],
            content,
        })
    }

    pub(crate) fn try_concat(&self, node: &Text) -> Option<Rc<dyn Node>> {
        if self.marks.len() == node.marks.len() {
            let text = format!("{}{}", self.content, node.content);

            Some(Rc::new(Self {
                marks: vec![],
                content: text,
            }))
        } else {
            None
        }
    }
}
