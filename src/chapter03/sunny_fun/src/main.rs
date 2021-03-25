// fn main() {
//     let xm = "xiaoming";
//     let xh = "xiaohong";
//     say_what(xm, hi);
//     say_what(xh, hello);
// }
// fn hi(name: &str) {
//     println!("Hi, {}.", name);
// }
// fn hello(name: &str){
//     println!("Hello, {}.", name);
// }
// fn say_what(name: &str, func: fn(&str)) {
//     func(name)
// }
fn sunny_foo(v:i32){
    println!("{}",v*v);
}
fn sunny_func(func: fn(v:i32),i:i32){
    func(i)
}

fn diverges_foo() -> ! {
    panic!("This call never returns.");
}
fn main(){
    let x:i32=3;
    sunny_func(sunny_foo,x);
    let _x: i32 = diverges_foo();
    let _y: String = diverges_foo();
}
// fn sunny_foo(v:i32)->i32{
//     v+1
// }
// fn main(){

//     let x = sunny_foo(8); // 9
//     println!("x={}",x);
//     println!("Hello, world!");

//     // let sunny_fun: fn(i32) -> i32 = sunny_foo;
//     //  let x = sunny_fun(8); // 9
//     //  println!("Hello, world!x={}",x);
// }
