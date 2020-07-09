use std::rc::Rc;
use crate::host::{Host, HostInstance};

pub trait HTMLElementTag<T> {
    fn new(host: Rc<dyn Host>, instance: Rc<dyn HostInstance>) -> T;
    fn create(host: Rc<dyn Host>) -> T;
}
