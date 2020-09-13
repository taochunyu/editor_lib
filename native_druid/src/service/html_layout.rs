use druid::piet::PietTextLayout;
use std::collections::HashMap;
use druid::{Point, Size};
use crate::widget::text::BasicText;
use crate::widget::rect::BasicRect;

#[derive(Eq, PartialEq, Hash)]
struct Word {
    content: String,
    font_size: u64,
}

struct HTMLLayoutFactory {
    word_layout_cache: HashMap<Word, PietTextLayout>,
}

impl HTMLLayoutFactory {
    pub fn new() -> Self {
        Self {
            word_layout_cache: HashMap::new(),
        }
    }

    pub fn create_layout(&mut self) -> HTMLLayout {
        HTMLLayout::new(self)
    }
}

enum HTMLBlockSize {
    FixWidth(f64),
}

struct HTMLBlockElement {
    tag_name: String,
    children: Vec<u64>,
    size: HTMLBlockSize,
    margin: (u64, u64, u64, u64),
    border: (u64, u64, u64, u64),
    padding: (u64, u64, u64, u64),
}

impl HTMLBlockElement {
    pub fn new() -> Self {
        Self {
            tag_name: String::new(),
            children: vec![],
            size: HTMLBlockSize::Auto,
            margin: (0, 0, 0, 0),
            border: (0, 0, 0, 0),
            padding: (0, 0, 0, 0)
        }
    }
}

struct HTMLInlineElement {
    tag_name: String,
    child: Option<u64>,
}

impl HTMLInlineElement {
    pub fn new() -> Self {
        Self {
            tag_name: String::new(),
            child: None,
        }
    }
}

struct HTMLText {
    content: String,
}

impl HTMLText {
    pub fn new(content: String) -> Self {
        Self {
            content,
        }
    }
}

enum HTMLNodeType {
    Block(HTMLBlockElement),
    Inline(HTMLInlineElement),
    Text(HTMLText),
}

struct HTMLNode {
    id: u64,
    parent_id: Option<u64>,
    widgets: Vec<u64>,
    node_type: HTMLNodeType,
}

impl HTMLNode {
    pub fn new(id: u64, node_type: HTMLNodeType) -> Self {
        Self {
            id,
            parent_id: None,
            widgets: vec![],
            node_type,
        }
    }

    pub fn parent_id(&self) -> Option<u64> {
        self.parent_id
    }

    pub fn node_type(&self) -> &HTMLNodeType {
        &self.node_type
    }

    pub fn node_type_mut(&mut self) -> &mut HTMLNodeType {
        &mut self.node_type
    }
}

enum HTMLWidget {
    Text(BasicText),
    Rect(BasicRect),
}

struct ChildWidget {
    coordinate: Point,
    widget: HTMLWidget,
}

struct HTMLLayout<'a> {
    factory: &'a mut HTMLLayoutFactory,
    id_counter: u64,
    nodes: HashMap<u64, HTMLNode>,
    widgets: HashMap<u64, ChildWidget>,
}

impl<'a> HTMLLayout<'a> {
    pub fn new(factory: &'a mut HTMLLayoutFactory) -> Self {
        let mut result = Self {
            factory,
            id_counter: 0,
            nodes: HashMap::new(),
            widgets: HashMap::new(),
        };
        let root = HTMLNodeType::Block(HTMLBlockElement::new());

        result.add_node(root);

        result
    }

    pub fn root(&self) -> &HTMLNode {
        self.nodes.get(&0).unwrap()
    }

    pub fn root_mut(&mut self) -> &mut HTMLNode {
        self.nodes.get_mut(&0).unwrap()
    }

    pub fn add_node(&mut self, node: HTMLNodeType) -> u64 {
        let id = self.id_counter;
        let node = HTMLNode::new(id, node);

        self.nodes.insert(id, node);
        self.id_counter += 1;

        id
    }

    pub fn remove_node(&mut self, id: u64) -> Result<(), String> {
        let mut parent_id = None;

        match self.nodes.get_mut(&id) {
            Some(node) => {
                parent_id = node.parent_id();

                match node.node_type() {
                    HTMLNodeType::Block(block) => {
                        block.children.iter().for_each(|child_id| {
                            self.remove_node(*child_id)?;
                        });
                    },
                    HTMLNodeType::Inline(inline) => {
                        inline.children.iter().for_each(|child_id| {
                            self.remove_node(*child_id)?;
                        });
                    },
                    HTMLNodeType::Text(_) => {}
                };
            },
            None => {
                return Err(format!("Cannot fin node {}.", id));
            }
        };

        if let Some(parent_id) = parent_id {
            match self.nodes.get_mut(&parent_id) {
                Some(parent) => {
                    match parent.node_type_mut() {
                        HTMLNodeType::Block(block) => {
                            if let Some(index) = block.children.iter().position(|child_id| *child_id == id) {
                                block.children.remove(index);
                            } else {
                                return Err(format!("Cannot find child {} in parent {}", id, parent_id));
                            }
                        },
                        HTMLNodeType::Inline(inline) => {
                            if let Some(index) = inline.children.iter().position(|child_id| *child_id == id) {
                                block.children.remove(index);
                            } else {
                                return Err(format!("Cannot find child {} in parent {}", id, parent_id));
                            }
                        },
                        HTMLNodeType::Text(_) => {}
                    }
                },
                None => {
                    return Err(format!("Cannot find parent node {}.", parent_id));
                },
            };
        }

        self.nodes.remove(&id);

        Ok(())
    }

    pub fn layout(&mut self, node: &HTMLNode, ctx: LayoutCtx) -> Size {
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;

        match node.node_type() {
            HTMLNodeType::Block(block) => {

            },
            HTMLNodeType::Inline(inline) => {

            },
            HTMLNodeType::Text(text) => {

            },
        }
    }
}

struct LayoutCtx<'a> {
    total_width: f64,
    used_width: f64,
    marks: Vec<&'a HTMLInlineElement>,
}

