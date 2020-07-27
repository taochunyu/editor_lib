pub mod state;
mod transform;
pub mod node_types;
pub mod node;
pub mod view;

use std::rc::Rc;
use crate::node::Node;

pub type Doc = Rc<dyn Node>;

pub type Position = usize;

#[cfg(test)]
pub mod test;
