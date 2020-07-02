use std::cell::RefCell;
use std::rc::Rc;

pub mod html;
pub mod instruction;
pub mod ui;

pub type DOM = Rc<RefCell<dyn html::Node>>;

#[cfg(test)]
mod tests {

}
