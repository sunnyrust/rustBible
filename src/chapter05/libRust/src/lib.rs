pub mod sunnylib;
#[no_mangle]
pub extern fn double_input(input: i32) -> i32 {
    input * 2
}
#[no_mangle]
pub extern "C" fn hello_rust(){
    println!("hello:C++>Rust");
    sunnylib::call_func();
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
