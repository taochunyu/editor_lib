type Start = usize;
type OldSize = usize;
type NewSize = usize;

type Range = (Start, OldSize, NewSize);

pub struct MapResult {
    offset: usize,
    deleted: usize,
    recover: Option<usize>,
}

struct StepMap {
    ranges: Vec<Range>,
    inverted: bool,
}

pub struct Mapping;