use std::cell::RefCell;
use std::rc::Rc;
use crate::html;

type Node = Rc<RefCell<dyn html::Node>>;

pub fn id(node: Node) -> html::NodeId {
    node.borrow().id()
}

pub fn append_child(node: Node, child: Node) -> Node {
    node.clone().borrow_mut().append_child(child);

    node.clone()
}
