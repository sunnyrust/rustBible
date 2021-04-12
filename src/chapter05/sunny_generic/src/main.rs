use std::ops::Add;
fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}


fn max<T>(array: &[T]) -> T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

fn main() {
    println!("{}", add(3i32, 9i32));
    println!("{}", add(3.1f64, 10.04f64));

    let array = [2.3, 4.9, 6.8, 3.1, 1.2];
    println!("max = {}", max(&array));
}
