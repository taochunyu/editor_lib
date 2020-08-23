use learn_rust_lib::{A, C};

trait B {
    fn foo(a: usize) -> usize;
}

impl B for A {
    fn foo(a: usize) -> usize {
        unimplemented!()
    }
}

fn main() {

}
