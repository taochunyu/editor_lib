pub struct A;

pub trait C {
    fn render() -> u32;
}

impl C for A {
    fn render() -> u32 {
        18
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
