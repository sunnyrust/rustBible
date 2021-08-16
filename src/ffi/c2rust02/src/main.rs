use chrono::prelude::*;

#[link(name = "greet", kind = "static")]
extern "C" {
   fn greet(name: *mut  std::os::raw::c_char) -> libc::c_int;
   fn triple_input(input: libc::c_int) -> libc::c_int;
}

fn greet_cpp(name: &str) {
    unsafe {
        let parm_name = name.as_ptr();
        let _i=greet(parm_name as *mut  i8);
        
    }
}
fn main() {
     greet_cpp("大家好，我是Sunny！");
    let input = 6;
    let output = unsafe { triple_input(input) };
    println!("{} * 3 = {}", input, output);
    println!("Hello, world!");

    let local: DateTime<Local> = Local::now();
    println!("{},{:#?}",local.format("%Z").to_string(), iana_time_zone::get_timezone().unwrap());

    println!("{},{:#?}",local.to_rfc3339(),local.timezone());
}
