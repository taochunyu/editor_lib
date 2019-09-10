#[allow(dead_code)]
enum Event {
    KeyDown(usize),
    KeyPress(String),
}

struct ResolvedPosition {
    position: usize,
    path: Vec<(usize, usize)>,
    parent_offset: usize,
}

impl Default for ResolvedPosition {
    fn default() -> ResolvedPosition {
        ResolvedPosition {
            position: 0,
            path: vec![],
            parent_offset: 0,
        }
    }
}

struct Message {
    resolved_from: ResolvedPosition,
    resolved_to: ResolvedPosition,
    event: Event,
}

impl Default for Message<> {
    fn default() -> Message {
        Message {
            resolved_from: ResolvedPosition::default(),
            resolved_to: ResolvedPosition::default(),
            event: Event::KeyDown(0),
        }
    }
}

trait Node {
    fn children(&self) -> Option<&Vec<Box<dyn Node>>>;
    fn size(&self) -> usize;
    fn to_string(&self) -> String;
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node>;
}

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
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node> {
        let event = &msg.event;
        let from_parent_offset = msg.resolved_from.parent_offset;
        let to_parent_offset = msg.resolved_to.parent_offset;

        match event {
            Event::KeyPress(text) => {
                Box::new(TextNode {
                    mark_list: None,
                    text_content: format!(
                        "{}{}{}",
                        self.text_content.get(0..from_parent_offset).unwrap_or(""),
                        text,
                        self.text_content.get(to_parent_offset..self.size()).unwrap_or(""),
                    ),
                })
            }
            _ => { self }
        }
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
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node> {
        let from = msg.resolved_from.position;
        let to = msg.resolved_to.position;
        let mut start: usize = msg.resolved_from.path[depth].1;
        let mut temp: Vec<Box<dyn Node>> = vec![];

        for child in self.children {
            let end = start + child.size();

            if start > to || end < from {
                temp.push(child);
            } else {
                temp.push(child.update(msg, depth + 1));
            }

            start = end;
        }

        Box::new(ParagraphNode {
            align: self.align,
            children: temp,
        })
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
    fn update(self: Box<Self>, msg: &Message, depth: usize) -> Box<dyn Node> {
        let from = msg.resolved_from.position;
        let to = msg.resolved_to.position;
        let mut start: usize = 0;
        let mut temp: Vec<Box<dyn Node>> = vec![];

        for child in self.children {
            let end = start + child.size();

            if start > to || end < from {
                temp.push(child);
            } else {
                temp.push(child.update(msg, depth + 1));
            }

            start = end;
        }

        Box::new(DocNode {
            children: temp,
        })
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
) -> Result<ResolvedPosition, &str> {
    if node.size() < position {
        return Err("Position out of Range.");
    }

    let mut path: Vec<(usize, usize)> = vec![];
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

                path.push((index, start + offset));

                let rem: usize = parent_offset - offset;

                if rem == 0 {
                    break;
                }

                if cursor.is_some() && cursor.unwrap().children().is_none() {
                    break;
                }

                parent_offset = rem - 1;
                start += offset + 1;
            }
        }
    }

    Ok(ResolvedPosition {
        position,
        path,
        parent_offset,
    })
}

fn main() {
    let doc: Box<dyn Node> = Box::new(DocNode {
        children: vec![build_paragraph("hi"), build_paragraph("hello")],
    });
    let msg = {
        let resolved_from = resolve_position(&doc, 8).unwrap_or(ResolvedPosition::default());
        let resolved_to = resolve_position(&doc, 8).unwrap_or(ResolvedPosition::default());

        Message {
            resolved_from,
            resolved_to,
            event: Event::KeyPress(String::from("u")),
        }
    };


    println!("{}", doc.update(&msg, 0).to_string());
}
