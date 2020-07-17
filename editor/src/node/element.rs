use std::rc::Rc;
use std::any::Any;
use crate::node::element_type::{ElementType, OuterDOM, ContentDOM};
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::view::View;

pub struct Element<T: ElementType> {
    attributes: Rc<T::Attributes>,
    children: Option<Rc<Fragment>>,
}

impl<T: ElementType> Node for Element<T> {
    fn type_name(&self) -> &str {
        T::name()
    }

    fn size(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.size() + 2,
            None => 1,
        }
    }

    fn content_size(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.size(),
            None => 0,
        }
    }

    fn child_count(&self) -> usize {
        match &self.children {
            Some(fragment) => fragment.count(),
            None => 0,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn cut(self: Rc<Self>, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        if from == 0 && to == self.content_size() {
            return Ok(self.clone() as Rc<dyn Node>);
        }

        match &self.children {
            Some(fragment) => {
                let result = fragment.cut(from, to)?;

                Ok(Rc::new(Self {
                    attributes: self.attributes.clone(),
                    children: Some(result),
                }))
            },
            None => Err(format!("cannot cut node without children")),
        }
    }

    fn index(&self, offset: usize) -> Result<usize, String> {
        match &self.children {
            Some(fragment) => fragment.index(offset),
            None => Err(format!("Cannot find offset index on element node without children.")),
        }
    }

    fn get_child(&self, index: usize) -> Result<Rc<dyn Node>, String> {
        match &self.children {
            Some(children) => children.get(index),
            None => Err(format!("Cannot get child on element node without children.")),
        }
    }

    fn replace_child(&self, index: usize, child: Rc<dyn Node>) -> Result<Rc<dyn Node>, String> {
        match &self.children {
            Some(children) => {
                let children = children.replace_child(index, child)?;

                self.replace_children(children)
            },
            None => Err(format!("Node without children cannot replace child."))
        }
    }

    fn children(&self) -> Option<Rc<Fragment>> {
        self.children.clone()
    }

    fn replace_children(&self, new_children: Rc<Fragment>) -> Result<Rc<dyn Node>, String> {
        Ok(Rc::new(Self {
            attributes: self.attributes.clone(),
            children: Some(new_children),
        }))
    }

    fn serialize(&self) -> String {
        match &self.children {
            Some(children) => {
                let open = format!("<{}>", T::name());
                let close = format!("</{}>", T::name());
                let content = children.to_string();

                format!("{}{}{}", open, content, close)
            },
            None => format!("<{} />", T::name()),
        }
    }

    fn render(self: Rc<Self<>>, view: Rc<View>) -> (OuterDOM, ContentDOM) {
        let attrs = self.clone().attributes.clone();

        T::render(view, self, attrs)
    }
}

impl<T: ElementType> Element<T> {
    pub(crate) fn new(attributes: Rc<T::Attributes>, children: Option<Rc<Fragment>>) -> Rc<Self> {
        Rc::new(Self { attributes, children })
    }
}
