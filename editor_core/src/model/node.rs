use crate::model::fragment::Fragment;
use crate::model::resolved_position::{resolve_position, ResolvedPosition};
use crate::model::slice::Slice;
use crate::nodes::text_node::Mark::Strong;
use std::any::Any;
use std::rc::Rc;
use crate::view::virtual_node::VirtualNode;

pub trait Node {
    fn type_name(&self) -> String;
    fn is_text(&self) -> bool {
        false
    }
    fn text_content(&self) -> &str {
        ""
    }
    fn slice_text_content(&self, from: usize, to: usize) -> Result<Rc<dyn Node>, String> {
        Err(format!("trait node slice_text_content must override!"))
    }
    fn need_join(&self, other: &dyn Node) -> bool {
        false
    }
    fn join(&self, other: &dyn Node) -> Result<Rc<dyn Node>, String> {
        Err(format!("trait node join must override!"))
    }
    fn is_leaf(&self) -> bool {
        false
    }
    fn content_size(&self) -> usize {
        0
    }
    fn border_size(&self) -> usize {
        if self.is_text() {
            0
        } else if self.is_leaf() {
            1
        } else {
            2
        }
    }
    fn to_string(&self, content: String) -> String;
    fn mark_to_string(&self) -> String;
    fn render(&self, children: Vec<Rc<VirtualNode>>) -> Rc<VirtualNode>;
}

impl dyn Node {
    pub fn is_mark_same(a: &dyn Node, b: &dyn Node) -> bool {
        a.mark_to_string() == b.mark_to_string()
    }
}

pub struct TreeNode {
    pub(crate) content: Option<Rc<Fragment>>,
    pub(crate) node: Rc<dyn Node>,
}

impl TreeNode {
    pub fn new(node: Rc<dyn Node>, content: Option<Rc<Fragment>>) -> Self {
        Self { node, content }
    }
    pub fn size(&self) -> usize {
        match &self.content {
            Some(content) => self.node.border_size() + self.node.content_size() + content.size,
            None => self.node.content_size(),
        }
    }
    pub fn content_size(&self) -> usize {
        if let Some(content) = &self.content {
            content.size
        } else {
            0
        }
    }
    pub fn child_count(&self) -> usize {
        if let Some(content) = &self.content {
            content.content.len()
        } else {
            0
        }
    }
    pub fn child(&self, index: usize) -> Result<Rc<TreeNode>, String> {
        if let Some(content) = &self.content {
            content.child(index)
        } else {
            Err(String::from("TreeNode::child: content is None"))
        }
    }
    pub fn need_join(&self, other: &Rc<Self>) -> bool {
        self.node.need_join(other.node.as_ref())
    }
    pub fn join(&self, other: &Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            node: Rc::clone(&self.node.join(other.node.as_ref()).unwrap()),
            content: match &self.content {
                Some(content) => Some(Rc::clone(content)),
                None => None,
            },
        })
    }
    pub fn is_text(&self) -> bool {
        self.node.is_text()
    }
    pub fn to_string(&self) -> String {
        let mut content_str = String::from("");

        if let Some(content) = &self.content {
            for tree_node in &content.content {
                content_str.push_str(tree_node.to_string().as_str());
            }
        }

        self.node.to_string(content_str)
    }
    pub fn render(&self) -> Rc<VirtualNode> {
        let mut content_virtual_node: Vec<Rc<VirtualNode>> = vec![];

        if let Some(content) = &self.content {
            for tree_node in &content.content {
                content_virtual_node.push(tree_node.render());
            }
        };

        self.node.render(content_virtual_node)
    }
    pub(crate) fn copy(&self, content: Option<Rc<Fragment>>) -> Rc<Self> {
        Rc::new(Self {
            node: Rc::clone(&self.node),
            content,
        })
    }
    pub(crate) fn cut(&self, from: usize, to: usize) -> Rc<Self> {
        if let Some(content) = &self.content {
            self.copy(Some(content.cut(from, to)))
        } else {
            if self.is_text() {
                Rc::new(Self::new(
                    self.node.slice_text_content(from, to).unwrap(),
                    None,
                ))
            } else {
                self.copy(None)
            }
        }
    }
}
