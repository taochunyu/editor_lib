mod builder;
mod options;
mod schema;
mod html_parser;
mod view;

use std::rc::Rc;
use std::cell::{RefCell, Ref};
use renderer::Renderer;
use renderer::html::div::HTMLDivElement;
use crate::view::View;
use crate::state::State;
use crate::schema::node_type::NodeType;
use crate::editor::options::Options;
use crate::state::transaction::Transaction;
use crate::node_types::root::Root;
use crate::node::Node;
use crate::node::element::Element;
use crate::node::text::Text;
use crate::schema::Schema;
use crate::Doc;
use crate::input::html_parser::HTMLParser;

pub struct Editor {
    options: Options,
    schema: Rc<RefCell<Schema>>,
    html_parser: Rc<HTMLParser>,
    state: State,
    view: View,
}

impl Editor {
    fn new(schema: Schema, state: State, view: View) -> Self {
        let options = Options::default();
        let schema = Rc::new(RefCell::new(schema));
        let html_parser = Rc::new(HTMLParser::new(schema.clone()));

        Self {
            options,
            schema,
            html_parser,
            state,
            view,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn create_transaction(&self) -> Transaction {
        self.state.create_transaction()
    }

    pub fn dispatch(&mut self, transaction: &Transaction) {
        self.state = self.state.apply(transaction);
        self.view.render(&self.state);
    }
}
