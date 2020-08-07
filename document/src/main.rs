use std::rc::Rc;
use renderer::Renderer;
use editor::view::View;
use document::Document;
use document::host::TestHost;
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ns = since_the_epoch.as_secs() as i64 * 1000000i64 + (since_the_epoch.subsec_nanos() as f64) as i64;
    ns
}

fn main() {
    let test_host = TestHost::new();
    let renderer = Rc::new(Renderer::new(test_host));

    let start = timestamp();

    let mut doc = Document::new(renderer);

    let end = timestamp();

    doc.trigger_test();
    println!("{}", end - start);
}