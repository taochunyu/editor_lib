#[allow(dead_code)]
trait Node {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>>;
    fn size(&self) -> usize;
    fn to_string(&self) -> String;
}

impl dyn Node {}

enum Mark {
    Strong,
    Em,
}

struct TextNode {
    mark_list: Option<Vec<Mark>>,
    text_content: String,
}

impl Node for TextNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        None
    }
    fn size(&self) -> usize {
        self.text_content.len()
    }
    fn to_string(&self) -> String {
        let mut result = String::from("<span data-marks=\"");
        match &self.mark_list {
            None => {}
            Some(marks) => {
                let mut divide = "";
                for mark in marks {
                    result.push_str(divide);
                    match mark {
                        Mark::Strong => result.push_str("strong"),
                        Mark::Em => result.push_str("em"),
                    }
                    divide = " ";
                }
            }
        };

        result.push_str("\">");
        result.push_str(self.text_content.as_str());
        result.push_str("</span>");
        result
    }
}

enum Align {
    Left,
    Center,
    Right,
}

struct ParagraphNode {
    align: Align,
    children: Vec<Box<dyn Node>>,
}

impl Node for ParagraphNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        if self.children.len() == 0 {
            None
        } else {
            Some(&self.children)
        }
    }
    fn size(&self) -> usize {
        self.children.iter().fold(0, |acc, x| acc + x.size()) + 2
    }
    fn to_string(&self) -> String {
        let mut result = String::from("<p data-align=\"");
        match self.align {
            Align::Left => result.push_str("left"),
            Align::Center => result.push_str("center"),
            Align::Right => result.push_str("right"),
        };
        result.push_str("\">");
        for child in &self.children {
            result.push_str(child.to_string().as_str());
        }
        result.push_str("</p>");
        result
    }
}

struct DocNode {
    children: Vec<Box<dyn Node>>,
}

impl Node for DocNode {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>> {
        if self.children.len() == 0 {
            None
        } else {
            Some(&self.children)
        }
    }
    fn size(&self) -> usize {
        self.children.iter().fold(0, |acc, x| acc + x.size())
    }
    fn to_string(&self) -> String {
        let mut result = String::from("<div class=\"editor\">");
        for child in &self.children {
            result.push_str(child.to_string().as_str());
        }
        result.push_str("</div>");
        result
    }
}

fn build_paragraph(content: &str) -> Box<dyn Node> {
    let text_node_box: Box<dyn Node> = Box::new(TextNode {
        mark_list: Some(vec![Mark::Strong]),
        text_content: String::from(content),
    });

    Box::new(ParagraphNode {
        align: Align::Left,
        children: vec![text_node_box],
    })
}

fn resolve_position(
    node: &Box<dyn Node>,
    position: usize,
) -> Result<Vec<(&Box<dyn Node>, usize, usize)>, &str> {
    if node.size() < position {
        return Err("Position out of Range.");
    }

    let mut path: Vec<(&Box<dyn Node>, usize, usize)> = vec![];
    let mut cursor: Option<&Box<dyn Node>> = Some(node);
    let mut start: usize = 0;
    let mut parent_offset: usize = position;

    while let Some(parent) = cursor {
        match parent.children() {
            None => cursor = None,
            Some(children) => {
                let mut index = 0;
                let mut offset = 0;

                for child in children {
                    if offset + child.size() > parent_offset {
                        cursor = Some(child);
                        break;
                    }
                    index += 1;
                    offset += child.size();
                }

                path.push((parent, index, start + offset));

                let rem: usize = parent_offset - offset;

                if rem == 0 {
                    break;
                }

                parent_offset = rem - 1;
                start += offset + 1;
            }
        }
    }

    Ok(path)
}

fn replace(
    from: Vec<(&Box<dyn Node>, usize, usize)>,
    to: Vec<(&Box<dyn Node>, usize, usize)>,
) {}

fn main() {
    let tree: Box<dyn Node> = Box::new(DocNode {
        children: vec![build_paragraph("hi"), build_paragraph("hello")],
    });

    for (node, index, parent_offset) in resolve_position(&tree, 11).unwrap_or(vec![]) {
        println!("{} {} {}", node.to_string(), index, parent_offset);
    }
}
