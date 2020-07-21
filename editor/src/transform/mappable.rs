use crate::Position;

const LOWER_16: usize = 0xffff;
const FACTOR_16: usize = 0x8000;

pub fn make_recover(index: usize, offset: usize) -> usize {
    index + offset * FACTOR_16
}

pub fn recover_index(value: usize) -> usize {
    value & LOWER_16
}

pub fn recover_offset(value: usize) -> usize {
    (value - (value & LOWER_16)) / FACTOR_16
}

type Start = usize;
type OldSize = usize;
type NewSize = usize;

pub type Range = (Start, OldSize, NewSize);

pub struct MapResult {
    position: Position,
    deleted: bool,
    recover: Option<usize>,
}

impl MapResult {
    pub fn new(position: Position, deleted: bool, recover: Option<usize>) -> Self {
        MapResult { position, deleted, recover }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn deleted(&self) -> bool {
        self.deleted
    }

    pub fn recover(&self) -> Option<usize> {
        self.recover
    }
}

#[derive(Clone)]
pub enum AssociatedSide {
    Left,
    Right,
}

impl Default for AssociatedSide {
    fn default() -> Self {
        AssociatedSide::Right
    }
}

pub trait Mappable {
    fn map(&self, pos: Position, assoc: AssociatedSide) -> Position;
    fn map_result(&self, pos: Position, assoc: AssociatedSide) -> MapResult;
}
