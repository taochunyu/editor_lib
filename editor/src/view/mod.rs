use std::cell::RefCell;
use std::rc::Rc;

mod node_view_description;
mod updater;

type Shared<T> = Rc<RefCell<T>>;