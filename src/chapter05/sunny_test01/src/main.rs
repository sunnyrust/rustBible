fn main() {
    println!("Hello, world!");
    let one_plus_one = stringify!(1 + 1);
    assert_eq!(one_plus_one, "1 + 2");
}
