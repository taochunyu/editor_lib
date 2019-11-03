pub(crate) struct Range {
    from: usize,
    to: usize,
}

impl Default for Range {
    fn default() -> Self {
        Range { from: 0, to: 0 }
    }
}
