#[link(name = "greet", kind = "static")]
extern "C" {
   fn greet(name: *mut  std::os::raw::c_char);
   fn triple_input(input: libc::c_int) -> libc::c_int;
}

fn greet_cpp(name: &str) {
    unsafe {
        let parm_name = name.as_ptr();
        greet(parm_name as *mut  i8);
    }
}
fn main() {
     greet_cpp("大家好，我是Sunny！");
    let input = 6;
    let output = unsafe { triple_input(input) };
    println!("{} * 3 = {}", input, output);
    println!("Hello, world!");
}
