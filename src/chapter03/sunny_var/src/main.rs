#[derive(Clone, Copy)]
struct A;
fn main() {
    // let i_age=35;
    // i_age = 40;

    // println!("{}",i_age);

    let a = A;
    let b = a;   // 这个是没有进行权利转移
    let c = a;      //  要是这两句进行换位还是会出现上面的错误。
  
    let v:i16 = 8;
    let v:&str =  "hello";
    let v:f64 = 0.618;
    println!("The value of v is: {}", v);

    let mut str_sunny = "Hello World";
    println!("str_sunny={}", str_sunny);
    str_sunny = str_sunny.len();
    println!("str_sunny={}", str_sunny);
    
 }
