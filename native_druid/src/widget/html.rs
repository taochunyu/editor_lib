use druid::{Point, WidgetPod, Widget, Size};
use crate::services::block_layout::Layout;
use std::collections::HashMap;

struct Tile {
    id: u64,
    point: Point,
    size: Size,
    widget: WidgetPod<T, Box<dyn Widget<T>>>,
}

struct HTML<T> {
    layout_service: Layout,
    tiles: HashMap<u64, Tile>
}