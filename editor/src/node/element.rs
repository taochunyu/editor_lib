use std::rc::Rc;
use std::any::Any;
use renderer::Renderer;
use renderer::html::node::HTMLNode;
use renderer::html::element::HTMLElement;
use crate::schema::node_type::NodeType;
use crate::node::Node;
use crate::node::fragment::Fragment;
use crate::view::View;

pub struct Element<T: NodeType> {
    attributes: Rc<T::Attributes>,
    children: Option<Rc<Fragment>>,
}

impl<T: NodeType> Node for Element<T> {
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

    fn render(self: Rc<Self>, renderer: Rc<Renderer>) -> (HTMLNode, Option<HTMLElement>) {
        // renderer.clone().log("node render", self.clone().serialize());

        let attrs = self.clone().attributes.clone();
        let (outer, content) = T::render(renderer, self, attrs);

        (outer.into(), content)
    }

    fn same_mark_up(self: Rc<Self>, other: Rc<dyn Node>) -> bool {
        if Rc::ptr_eq(&(self.clone() as Rc<dyn Node>), &other) {
            true
        } else if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Rc::ptr_eq(&self.attributes, &other.attributes) || self.attributes.as_ref() == other.attributes.as_ref()
        } else {
            false
        }
    }

    fn value_eq(self: Rc<Self>, other: Rc<dyn Node>) -> bool {
        if Rc::ptr_eq(&(self.clone() as Rc<dyn Node>), &other) {
            return true;
        }

        self.clone().same_mark_up(other.clone()) && self.children_eq(other)
    }
}

impl<T: NodeType> Element<T> {
    pub(crate) fn new(attributes: Rc<T::Attributes>, children: Option<Rc<Fragment>>) -> Rc<Self> {
        Rc::new(Self { attributes, children })
    }

    fn children_eq(&self, other: Rc<dyn Node>) -> bool {
        match (self.children(), other.children()) {
            (Some(self_children), Some(other_children)) => {
                self_children.value_eq(other_children)
            },
            _ => false
        }
    }
}
