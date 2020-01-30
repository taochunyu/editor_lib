use crate::virtual_node::VirtualNode;
use std::rc::Rc;

pub trait Component {
//    type Input;

//    fn render(&self) -> Rc<VirtualNode>;

    fn init() -> Self;
}

trait Share {
    fn share(self) -> Self;
}

struct Demo;

impl Component for Demo {
//    type Input = i32;
fn init() -> Self {
    Demo
}

}

fn t() -> Box<dyn Share> {

}