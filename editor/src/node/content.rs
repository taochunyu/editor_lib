use crate::node::fragment::Fragment;
use std::rc::Rc;
use std::ops::Deref;

pub enum Content {
    Nodes(Fragment),
    Text(String),
    Empty,
}

impl Content {
    pub(crate) fn size(&self) -> usize {
        match self {
            Content::Nodes(fragment) => fragment.size(),
            Content::Text(str) => str.len(),
            Content::Empty => 0,
        }
    }

    pub(crate) fn cut(content: &Rc<Self>, from: usize, to: usize) -> Result<Rc<Self>, String> {
        match content.deref() {
            Content::Nodes(nodes) => {
                let new_nodes = nodes.cut(from, to)?;

                Ok(Rc::new(Content::Nodes(new_nodes)))
            },
            Content::Text(text) => match text.get(from..to) {
                Some(slice) => Ok(Rc::new(Content::Text(String::from(slice)))),
                None => Err(format!("rang {}..{} outside of text content", from, to)),
            },
            Content::Empty => Ok(Rc::new(Content::Empty)),
        }
    }
}
