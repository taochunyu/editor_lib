use std::rc::Rc;
use std::any::Any;
use crate::node::fragment::Fragment;
use crate::view::View;
use crate::node::Node;
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use renderer::html::text::HTMLTextNode;

const NAME: &'static str = "text";

pub struct Text {
    marks: Vec<u64>,
    content: String,
}

impl Node for Text {
    fn type_name(&self) -> &str {
        NAME
    }

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

    fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        if from == 0 && to == self.size() {
            return Ok(self.clone() as Rc<dyn Node>);
        }

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

    fn replace_child(&self, _: usize, _: Rc<dyn Node>) -> Result<Rc<dyn Node>, String> {
        Err(format!("Text node cannot replace child."))
    }

    fn children(&self) -> Option<Rc<Fragment>> {
        None
    }

    fn replace_children(&self, _: Rc<Fragment>) -> Result<Rc<dyn Node>, String> {
        Err(format!("Text node cannot replace children."))
    }

    fn serialize(&self) -> String {
        match self.marks.len() {
            0 => self.content.clone(),
            _ => format!("<span>{}</span>", self.content)
        }
    }

    fn render(self: Rc<Self>, _renderer: Rc<Renderer>) -> (HTMLNode, Option<HTMLElement>) {
        unreachable!("Render text node should use render_text method.")
    }

    fn same_mark_up(self: Rc<Self>, other: Rc<dyn Node>) -> bool {
        other.is_text()
    }

    fn value_eq(self: Rc<Self>, other: Rc<dyn Node>) -> bool {
        if let Some(other) = other.as_text() {
            self.content == other.content
        } else {
            false
        }
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

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn render_text(&self, renderer: Rc<Renderer>) -> HTMLTextNode {
        renderer.create_text_node(self.content.clone().as_str())
    }
}
