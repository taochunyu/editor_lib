#[cfg(test)]
mod test {
    use editor_core::pre::one_test;

    #[test]
    fn test_bootstrap() {
        one_test(8, 8);
    }
}
