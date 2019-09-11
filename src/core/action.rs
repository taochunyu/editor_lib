use crate::core::node::ResolvedPosition;

pub enum Event {
    KeyDown(usize),
    KeyPress(String),
}

pub struct Message {
    pub resolved_from: ResolvedPosition,
    pub resolved_to: ResolvedPosition,
    pub event: Event,
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