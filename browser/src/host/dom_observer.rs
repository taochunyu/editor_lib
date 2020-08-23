use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MutationObserver, MutationObserverInit, MutationRecord};
use editor::view::view_desc::ViewDesc;
use std::cmp;
use std::rc::{Rc, Weak};
use editor::editor::Editor;
use renderer::html::node::HTMLNode;
use crate::host::dom_pointer::DOMPointer;
use editor::input::html_parser::{ParseResult, HTMLParser};
use crate::host::any_node::AnyNode;
use std::cell::RefCell;
use editor::state::text_selection::TextSelection;
use editor::node::slice::Slice;

struct MutationObserverWrapper {
    node: web_sys::Node,
    options: MutationObserverInit,
    observer: Option<MutationObserver>,
}

impl MutationObserverWrapper {
    pub fn new(node: web_sys::Node, options: MutationObserverInit) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            node,
            options,
            observer: None,
        }))
    }

    pub fn set_observer(wrapper: &Rc<RefCell<Self>>, observer: MutationObserver) {
        wrapper.borrow_mut().observer = Some(observer);
    }

    pub fn start(wrapper: &Rc<RefCell<Self>>) {
        let wrapper = wrapper.borrow();

        if let Some(observer) = &wrapper.observer {
            observer.observe_with_options(&wrapper.node, &wrapper.options);
        }
    }

    pub fn stop(wrapper: &Rc<RefCell<Self>>) {
        if let Some(observer) = &wrapper.borrow().observer {
            observer.disconnect();
        }
    }
}

pub struct DOMObserver {
    editor: Rc<RefCell<Editor>>,
    observer: Rc<RefCell<MutationObserverWrapper>>,
}

struct MutationRegisterResult {
    node: HTMLNode,
    from: usize,
    to: usize,
    type_over: bool,
}

impl DOMObserver {
    pub fn new(node: web_sys::Node, editor: Rc<RefCell<Editor>>) -> Self {
        let mut dom_observer = Self {
            editor: editor.clone(),
            observer: MutationObserverWrapper::new(node, Self::create_observer_options()),
        };

        dom_observer.create_observer();

        dom_observer
    }

    pub fn start(&self) {
        MutationObserverWrapper::start(&self.observer);
    }

    pub fn stop(&self) {
        MutationObserverWrapper::stop(&self.observer);
    }

    fn flush(
        editor: &Rc<RefCell<Editor>>,
        observer: &Rc<RefCell<MutationObserverWrapper>>,
        mutations: Vec<MutationRecord>
    ) {
        let mut node: Option<HTMLNode> = None;
        let mut from: Option<usize> = None;
        let mut to: Option<usize> = None;
        let mut type_over = false;

        for mutation in mutations {
            if let Some(result) = Self::register_mutation(mutation) {
                node = Some(result.node);
                from = Some(if let Some(from) = from { cmp::min(result.from, from) } else { result.from });
                to = Some(if let Some(to) = to { cmp::max(result.to, to) } else { result.to });
                type_over = result.type_over;
            }
        }

        match (node, from, to) {
            (Some(node), Some(from), Some(to)) => {
                Self::read_dom_change(editor, observer, node, from, to, type_over)
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
                    let to = view_desc.clone().pos_at_end();
                    let type_over = mutation.target().map(|x| x.node_value()) == Some(mutation.old_value());

                    Some(MutationRegisterResult {
                        node: view_desc.dom().clone(),
                        from,
                        to,
                        type_over,
                    })
                },
                _ => None,
            }
        } else {
            None
        }

    }

    fn read_dom_change(
        editor: &Rc<RefCell<Editor>>,
        observer: &Rc<RefCell<MutationObserverWrapper>>,
        node: HTMLNode,
        from: usize,
        to: usize,
        type_over: bool
    ) {
        let mut editor = editor.borrow_mut();
        let root = Box::new(AnyNode::from(node));
        let node = match editor.parser().parse(root) {
            ParseResult::Node(node) => node,
        };
        let mut tr = editor.create_transaction();
        let selection = TextSelection::new(editor.state().doc().clone(), from, to).unwrap();
        let selection = Rc::new(selection);

        tr.set_selection(Some(selection)).replace_selection(Slice::from(node.clone()));

        web_sys::console::log_1(&format!("{} {} {} {}", from, to, type_over, node.serialize()).into());

        MutationObserverWrapper::stop(observer);
        editor.dispatch(&tr);
        MutationObserverWrapper::start(observer);
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

    fn create_observer(&self) {
        let editor = self.editor.clone();
        let observer_wrapper = self.observer.clone();
        let callback = move |processed_mutations: js_sys::Array, observer: MutationObserver| {
            let mut mutations: Vec<MutationRecord> = vec![];

            for mutation in processed_mutations.to_vec() {
                mutations.push(mutation.dyn_into::<MutationRecord>().unwrap());
            }

            let pending_mutations = observer.take_records();

            for mutation in pending_mutations.to_vec() {
                mutations.push(mutation.dyn_into::<MutationRecord>().unwrap());
            }

            Self::flush(&editor, &observer_wrapper, mutations);
        };
        let mutation_callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(_, _)>);
        let observer = MutationObserver::new(mutation_callback.as_ref().unchecked_ref()).unwrap();

        mutation_callback.forget();

        MutationObserverWrapper::set_observer(&self.observer, observer)
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
