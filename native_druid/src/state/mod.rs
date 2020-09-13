use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct State {
    show_sidebar: bool,
}

impl State {
    pub fn new() -> Self {
        Self { show_sidebar: false }
    }
}
