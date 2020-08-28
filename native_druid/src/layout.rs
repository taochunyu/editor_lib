use std::collections::HashMap;

type Width = u64;
type Height = u64;

enum Node {
    Block(Width, Height),
    Text(String),
}

struct TextLayout {
    words: HashMap<String, u64>,
}

impl TextLayout {

}
