use druid::Size;
use druid::piet::{PietTextLayout, PietText, FontFamily, TextLayout, Text, TextLayoutBuilder};
use std::collections::HashMap;

#[derive(Clone)]
pub enum Node {
    Block(Block),
    Text(String),
}

#[derive(Clone)]
pub struct Block {
    size: Size,
    text: Option<String>,
}

impl Block {
    pub fn new(size: Size, text: Option<String>) -> Self {
        Self { size, text }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn text(&self) -> Option<String> {
        self.text.clone()
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Word {
    content: String,
    font_size: u64,
}

impl Word {
    pub fn new(content: String, font_size: u64) -> Self {
        Self { content, font_size }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn font_size(&self) -> u64 {
        self.font_size
    }
}

pub struct Row {
    pub height: f64,
    pub blocks: Vec<((f64, f64), Block)>,
}

pub struct LayoutContext {
    width: f64,
    rows: Vec<Row>,
    flow: (f64, f64),
}

impl LayoutContext {
    pub fn new(width: f64) -> Self {
        Self {
            width,
            rows: vec![Row { height: 0.0, blocks: vec![] }],
            flow: (0.0, 0.0),
        }
    }

    pub fn rows(&self) -> &Vec<Row> {
        &self.rows
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.rows.iter().fold(0.0, |init, row| init + row.height)
    }

    pub fn rest_width_in_current_row(&self) -> f64 {
        self.width - self.flow.0
    }

    pub fn append_block(&mut self, block: Block) {
        let (x, y) = self.flow;
        let Size { width, height } = block.size();

        if x == width {
            let point = (0.0, y + self.rows.last().unwrap().height);

            self.rows.push(Row {
                height,
                blocks: vec![(point, block)],
            });

            self.flow = point;
        } else {
            let point = self.flow;

            if height > self.rows.last().unwrap().height {
                self.rows.last_mut().unwrap().height = height;
            }

            if x + width > self.width {
                self.rows.push(Row { height: 0.0, blocks: vec![(point, block)] });
                self.flow = (0.0, self.height());
            } else {
                self.flow = (x + width, self.flow.1);
                self.rows.last_mut().unwrap().blocks.push((point, block));
            }
        }
    }
}

pub struct Layout {
    word_layout_cache: HashMap<Word, PietTextLayout>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            word_layout_cache: HashMap::new(),
        }
    }

    pub fn layout(&mut self, piet_text: &mut PietText, nodes: Vec<Node>, indent: bool) -> LayoutContext {
        let mut ctx = LayoutContext::new(720.0);

        if indent {
            ctx.append_block(Block::new(Size::new(15.0, 0.0), None))
        }

        for node in nodes {
            match node {
                Node::Block(block) => ctx.append_block(block),
                Node::Text(text) => {
                    let blocks = self.convert_text_to_blocks(piet_text, text, 18);

                    for block in blocks {
                        ctx.append_block(block);
                    }
                }
            }
        }

        ctx
    }

    pub fn convert_text_to_blocks(
        &mut self,
        piet_text: &mut PietText,
        text: String,
        font_size: u64,
    ) -> Vec<Block> {
        let words = Self::split_text(text);

        words.iter().map(|word| {
            let content = String::from(word);

            self.create_text_block(piet_text, content, font_size)
        }).collect()
    }

    fn create_text_block(
        &mut self,
        piet_text: &mut PietText,
        content: String,
        font_size: u64,
    ) -> Block {
        let word = Word::new(content, font_size);

        if let Some(layout) = self.word_layout_cache.get(&word) {
            Block::new(layout.size(), Some(word.content.clone()))
        } else {
            let layout = piet_text
                .new_text_layout(word.content().as_str())
                .font(FontFamily::SYSTEM_UI, word.font_size() as f64)
                .build().unwrap();

            let block = Block::new(layout.size(), Some(word.content.clone()));

            self.word_layout_cache.insert(word, layout);

            block
        }
    }

    fn split_text(text: String) -> Vec<String> {
        let mut result = vec![];

        for word in text.split(' ') {
            result.push(String::from(word));
            result.push(String::from(" "));
        }

        result.pop();

        result
    }
}

#[cfg(test)]
mod test {
    use crate::service::block_layout::Layout;

    #[test]
    fn create() {
        Layout::new();
    }
}

