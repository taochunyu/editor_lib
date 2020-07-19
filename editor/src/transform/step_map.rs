use crate::Position;
use crate::transform::step::StepResult;

const LOWER_16: usize = 0xffff;
const FACTOR_16: usize = 0x8000;

fn make_recover(index: usize, offset: usize) -> usize {
    index + offset * FACTOR_16
}
fn recover_index(value: usize) -> usize {
    value & LOWER_16
}
fn recover_offset(value: usize) -> usize {
    (value - (value & LOWER_16)) / FACTOR_16
}

type Start = usize;
type OldSize = usize;
type NewSize = usize;

type Range = (Start, OldSize, NewSize);

pub struct MapResult {
    position: Position,
    deleted: bool,
    recover: Option<usize>,
}

impl MapResult {
    fn new(position: Position, deleted: bool, recover: Option<usize>) -> Self {
        MapResult { position, deleted, recover }
    }
}

pub struct StepMap {
    ranges: Vec<Range>,
    inverted: bool,
    recover: Option<usize>,
}

impl StepMap {
    pub fn map_result(&self, pos: Position, assoc: Option<usize>) -> MapResult {
        let mut diff: usize = 0;

        for (index, range) in self.ranges.iter().enumerate() {
            let start = range.0 - if self.inverted { diff } else { 0 };

            if start > pos {
                break;
            }

            let old_size = if self.inverted { range.2 } else { range.1 };
            let new_size = if self.inverted { range.2 } else { range.1 };
            let end = start + old_size;

            if pos <= end {
                let side = match (old_size, pos) {
                    (0, _) => assoc,
                    (_, pos) if pos == start => None,
                    (_, pos) if pos == end => Some(1),
                    _ => assoc,
                };
                let result = start + diff + if side.is_none() { 0 } else { new_size };
                let assoc = if assoc.is_none() { start } else { end };
                let recover = if pos == assoc {
                    None
                } else {
                    Some(make_recover(index, pos - start))
                };

                return MapResult::new(result, pos == assoc, recover);
            }

            diff += new_size - old_size;
        }

        MapResult::new(pos, false, None)
    }

    pub fn recover(&self, value: usize) -> usize {
        let mut diff = 0;

        let index = recover_index(value);

        if !self.inverted {
            for i in 0..index {
                diff += self.ranges[i].2 - self.ranges[i].1;
            }
        }

        self.ranges[index].0 + diff + recover_offset(value)
    }
}

pub struct Mapping  {
    maps: Vec<StepMap>,
    from: usize,
    to: usize,
    mirror: Option<Vec<usize>>,
}

impl Mapping {
    pub fn new() -> Mapping {
        Mapping {
            maps: vec![],
            from: 0,
            to: 0,
            mirror: None,
        }
    }

    pub fn get_mirror(&self, n: usize) -> Option<usize> {
        let mirror = self.mirror.as_ref()?;

        for (index, value) in mirror.iter().enumerate() {
            if value == &n {
                let index = if index % 2 == 0 { index + 1 } else { index - 1 };

                return Some(mirror.get(index)?.clone());
            }
        }

        None
    }

    pub fn map_result(&self, position: Position, assoc: Option<usize>) -> MapResult {
        let mut pos = position;
        let mut deleted = false;
        let mut index = self.from;

        while index < self.to {
            let map_result = self.maps[index].map_result(pos, assoc);

            if let Some(recover) = map_result.recover {
                if let Some(corr) = self.get_mirror(index) {
                    if corr > index && corr < self.to {
                        index = corr;
                        pos = self.maps[corr].recover(recover);

                        continue;
                    }
                }
            }

            pos = map_result.position;
            deleted = map_result.deleted;
        }

        MapResult::new(position, deleted, None)
    }

    pub fn map(&self, position: Position, assoc: Option<usize>) -> usize {
        self.map_result(position, assoc).position
    }
}
