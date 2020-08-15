use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserver, MutationObserverInit, MutationRecord};
use editor::view::view_desc::ViewDesc;
use std::cmp;
use std::rc::Rc;
use crate::host::dom_pointer::DOMPointer;

pub struct DOMObserver {
    node: web_sys::Node,
    observer_options: MutationObserverInit,
    observer: MutationObserver,
}

struct MutationRegisterResult {
    from: usize,
    to: usize,
    type_over: bool,
}

impl MutationRegisterResult {
    fn new(from: usize, to: usize, type_over: bool) -> Self {
        Self { from, to, type_over }
    }
}

impl DOMObserver {
    pub fn new(node: web_sys::Node) -> Self {
        Self {
            node,
            observer_options: Self::create_observer_options(),
            observer: Self::create_observer(),
        }
    }

    pub fn start(&self) {
        self.observer.observe_with_options(&self.node, &self.observer_options);
    }

    fn flush(mutations: Vec<MutationRecord>) {
        let mut from: Option<usize> = None;
        let mut to: Option<usize> = None;
        let mut type_over = false;

        for mutation in mutations {
            if let Some(result) = Self::register_mutation(mutation) {
                from = Some(if let Some(from) = from { cmp::min(result.from, from) } else { result.from });
                to = Some(if let Some(to) = to { cmp::max(result.to, to) } else { result.to });
                type_over = result.type_over;
            }
        }

        match (from, to) {
            (Some(from), Some(to)) => {
                Self::read_dom_change(from, to, type_over)
            },
            _ => {},
        }
    }

    fn register_mutation(mutation: MutationRecord) -> Option<MutationRegisterResult> {
        let view_desc = Self::nearest_view_desc(mutation.target());

        if let Some(view_desc) = view_desc {
            match mutation.type_().as_str() {
                "characterData" => {
                    let from = view_desc.clone().pos_at_start();
                    let to = view_desc.pos_at_end();
                    let type_over = mutation.target().map(|x| x.node_value()) == Some(mutation.old_value());

                    Some(MutationRegisterResult::new(from, to, type_over))
                },
                _ => None,
            }
        } else {
            None
        }

    }

    fn read_dom_change(from: usize, to: usize, type_over: bool) {
        web_sys::console::log_1(&format!("{} {} {}", from, to, type_over).into())
    }

    fn create_observer_options() -> MutationObserverInit {
        let mut observer_options = MutationObserverInit::new();

        observer_options
            .child_list(true)
            .character_data(true)
            .character_data_old_value(true)
            .attributes(true)
            .attribute_old_value(true)
            .subtree(true);

        observer_options
    }

    fn create_observer() -> MutationObserver {
        let callback = move |processed_mutations: js_sys::Array, observer: MutationObserver| {
            let mut mutations: Vec<MutationRecord> = vec![];

            for mutation in processed_mutations.to_vec() {
                mutations.push(mutation.dyn_into::<MutationRecord>().unwrap());
            }

            let pending_mutations = observer.take_records();

            for mutation in pending_mutations.to_vec() {
                mutations.push(mutation.dyn_into::<MutationRecord>().unwrap());
            }

            Self::flush(mutations);
        };
        let mutation_callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_, _)>);
        let observer = MutationObserver::new(mutation_callback.as_ref().unchecked_ref()).unwrap();

        mutation_callback.forget();

        observer
    }

    fn nearest_view_desc(node: Option<web_sys::Node>) -> Option<Rc<dyn ViewDesc>> {
        let mut cursor = node;

        while let Some(node) = cursor {
            if let Some(extra_info) = DOMPointer::get(&node) {
                return Some(extra_info.get_view_desc())
            }

            cursor = node.parent_node();
        }

        None
    }
}
