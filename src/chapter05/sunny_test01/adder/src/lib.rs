#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 * 2, 4);
    }

    #[test]
    fn do_fail() {
        assert_eq!(2 -2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
