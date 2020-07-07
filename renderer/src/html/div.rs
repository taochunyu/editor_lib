use crate::html::{HtmlNode, Tag};
use crate::host::Host;

pub struct Div;

impl<H: Host> Tag<H> for Div {
    fn create(host: &H) -> HtmlNode<H> {
        let instance = host.create_instance("div");

        HtmlNode { instance }
    }
}
