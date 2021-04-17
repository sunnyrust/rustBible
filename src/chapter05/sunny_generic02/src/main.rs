#[allow(dead_code)]
struct Point<T,U> {
    x: T,
    y: U
}
fn main() {
    let _int_both = Point { x: 0, y: 0 };
    let _float_oboth = Point { x: 0.0, y: 0.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
    println!("Hello, world!");
}
