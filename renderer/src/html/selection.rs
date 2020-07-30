use crate::html::node::HTMLNode;

#[derive(Clone, Eq, PartialEq)]
pub struct Point {
    node: HTMLNode,
    offset: usize,
}

pub struct Selection {
    anchor: Point,
    head: Point,
}

impl Selection {
    pub fn anchor(&self) -> Point {
        self.anchor.clone()
    }

    pub fn head(&self) -> Point {
        self.head.clone()
    }
}
