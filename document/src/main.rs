use std::rc::Rc;
use renderer::Renderer;
use renderer::host::debug_host::DebugHost;
use editor::view::View;
use document::Document;
use std::time::{SystemTime, UNIX_EPOCH};
use editor::node::Node;

fn timestamp_ms() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64
}

fn main() {
    let test_host = DebugHost::new();
    let renderer = Rc::new(Renderer::new(test_host));

    let start = timestamp_ms();

    let mut doc = Document::new(renderer);

    let end = timestamp_ms();

    doc.trigger_test();
    println!("{} ", end - start);
}