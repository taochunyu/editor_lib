use crate::html::{HtmlNode, Tag};
use crate::host::Host;

pub struct P;

impl<H: Host> Tag<H> for P {
    fn create(host: &H) -> HtmlNode<H> {
        let instance = host.create_instance("p");

        HtmlNode { instance }
    }
}
