use std::rc::Rc;
use std::any::Any;
use crate::{Doc, Position};
use crate::node::path::Path;
use crate::state::selection::Selection;
use crate::transform::mapping::Mapping;
use crate::transform::mappable::{Mappable, AssociatedSide};
use crate::node::slice::Slice;
use crate::transform::Transform;

const TEXT_SELECTION_NAME: &'static str = "text_selection";

pub struct TextSelection {
    doc: Doc,
    anchor: Path,
    head: Path,
}

impl Selection for TextSelection {
    fn name() -> &'static str {
        TEXT_SELECTION_NAME
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn map(&self, doc: Doc, mapping: Mapping) -> Result<Rc<dyn Selection>, String> {
        let anchor = mapping.map(self.anchor.offset(), AssociatedSide::Right);
        let head = mapping.map(self.head.offset(), AssociatedSide::Right);

        Ok(Rc::new(Self::new(doc, anchor, head)?))
    }

    fn content(&self) -> Result<Slice, String> {
        let (from, to) = if self.anchor.offset() < self.head.offset() {
            (&self.anchor, &self.head)
        } else {
            (&self.head, &self.anchor)
        };

        self.doc.clone().slice(from.offset(), to.offset())
    }

    fn eq(&self, other: Rc<dyn Selection>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            Rc::ptr_eq(&self.doc, &other.doc)
                && self.anchor.offset() == other.anchor.offset()
                && self.head.offset() == other.head.offset()
        } else {
            false
        }
    }

    fn replace(&self, tr: &mut Transform, slice: Slice) {
        tr.replace(self.from().offset(), self.to().offset(), slice);
    }
}

impl TextSelection {
    pub fn new(doc: Doc, anchor: Position, head: Position) -> Result<Self, String> {
        if anchor > doc.content_size() || head > doc.content_size() {
            Err(format!(""))
        } else {
            let anchor = doc.clone().resolve(anchor)?;
            let head = doc.clone().resolve(head)?;

            Ok(Self { doc, anchor, head })
        }
    }

    pub fn from(&self) -> &Path {
        if self.anchor.offset() < self.head.offset() {
            &self.anchor
        } else {
            &self.head
        }
    }

    pub fn to(&self) -> &Path {
        if self.anchor.offset() < self.head.offset() {
            &self.head
        } else {
            &self.anchor
        }
    }
}
