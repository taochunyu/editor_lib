use crate::transform::step_map::StepMap;
use crate::Position;
use crate::transform::mappable::{MapResult, Mappable, AssociatedSide};

pub struct Mapping  {
    maps: Vec<StepMap>,
    mirror: Option<Vec<(usize, Option<usize>)>>,
    from: usize,
    to: usize,
}

impl Mappable for Mapping {
    fn map(&self, position: Position, assoc: AssociatedSide) -> usize {
        self.map_result(position, assoc).position()
    }

    fn map_result(&self, position: Position, assoc: AssociatedSide) -> MapResult {
        let mut pos = position;
        let mut deleted = false;
        let mut index = self.from;

        while index < self.to {
            let map_result = self.maps[index].map_result(pos, assoc.clone());

            if let Some(recover) = map_result.recover() {
                if let Some(corr) = self.get_mirror(index) {
                    if corr > index && corr < self.to {
                        index = corr;
                        pos = self.maps[corr].recover(recover);

                        continue;
                    }
                }
            }

            pos = map_result.position();
            deleted = map_result.deleted();
        }

        MapResult::new(position, deleted, None)
    }
}

impl Mapping {
    pub fn new(maps: Vec<StepMap>, mirror: Option<Vec<(usize, Option<usize>)>>, from: usize, to: usize) -> Mapping {
        Mapping { maps, mirror, from, to }
    }

    pub fn add_map(&mut self, map: StepMap, mirrors: Option<usize>) {
        self.maps.push(map);
        self.to = self.maps.len();

        if self.mirror.is_some() {
            self.set_mirror(self.to - 1, mirrors);
        }
    }

    pub fn set_mirror(&mut self, n: usize, m: Option<usize>) {
        if let Some(mirror) = &mut self.mirror {
            mirror.push((n, m));
        } else {
            self.mirror = Some(vec![(n, m)]);
        }
    }

    pub fn get_mirror(&self, n: usize) -> Option<usize> {
        let mirror = self.mirror.as_ref()?;

        for (a, b) in mirror {
            if a == &n {
                return b.clone()
            }

            if let Some(v) = b {
                if v == &n {
                    return Some(a.clone())
                }
            }
        }

        None
    }
}
