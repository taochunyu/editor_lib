pub mod doc;
pub mod n;
mod node;
mod position;
mod schema;
pub mod slice;
pub mod view;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
