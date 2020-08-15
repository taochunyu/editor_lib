pub mod state;
pub mod node_types;
pub mod node;
pub mod view;
pub mod schema;

mod input;
mod transform;

use std::rc::Rc;
use crate::node::Node;

pub type Doc = Rc<dyn Node>;
pub type Position = usize;

pub struct Editor {

}

impl Editor {

}



#[cfg(test)]
pub mod test;
