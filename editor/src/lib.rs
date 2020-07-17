use crate::node::Node;

mod transform;
pub mod node_types;
pub mod node;
pub mod view;

pub type Doc = dyn Node;

pub type Position = usize;

#[cfg(test)]
mod test;
