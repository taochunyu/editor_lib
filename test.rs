trait A {
    fn a(&self);
}

struct B;

impl A for B {
    fn a(&self) {
        println!("123");
    }
}

fn main() {
    let b = B {};

    b.a();
}