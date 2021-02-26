fn show(arr: &[u8]) {
    for i in arr {
        print!("{} ", i);
    }
    println!("");
}
fn main() {
    let x: [u8; 3] = [1, 2, 3];
    let x_slice=&x[..];
    show(x_slice);
    let y: [u8; 4] = [1, 2, 3, 4];
    show(&y[..]);
}
