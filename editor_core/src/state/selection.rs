use crate::state::range::Range;

pub struct Selection {
    ranges: Vec<Range>,
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            ranges: vec![Range::default()],
        }
    }
}
