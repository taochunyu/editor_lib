use crate::transform::mappable::{Range, MapResult, Mappable, AssociatedSide, make_recover, recover_index, recover_offset};
use crate::Position;

pub struct StepMap {
    ranges: Vec<Range>,
    inverted: bool,
}

impl Mappable for StepMap {
    fn map(&self, pos: usize, assoc: AssociatedSide) -> usize {
        self.map_result(pos, assoc).position()
    }

    fn map_result(&self, pos: Position, assoc: AssociatedSide) -> MapResult {
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
                    (_, pos) if pos == start => AssociatedSide::Left,
                    (_, pos) if pos == end => AssociatedSide::Right,
                    _ => assoc,
                };
                let result = start + diff + match side {
                    AssociatedSide::Left => 0,
                    AssociatedSide::Right => new_size,
                };
                let assoc = match side {
                    AssociatedSide::Left => start,
                    AssociatedSide::Right => end,
                };
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
}

impl StepMap {
    pub fn new(ranges: Vec<Range>, inverted: bool) -> StepMap {
        StepMap { ranges, inverted }
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
