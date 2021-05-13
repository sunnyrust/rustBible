extern "C" { 
    fn abs(input: i32) -> i32;
    fn write(fd: i32, data: *const u8, len: usize) -> isize;
}

#[test]
fn test_ffi() {
    let data = b"Hello, world!\n"; 
    unsafe {
        assert_eq!(abs(-5), 5);
        // 写入到标准输出
        write(1, data.as_ptr(), data.len());
    }
}
fn main() {
    println!("Hello, world!");
    let data = b"Hello, world!\n"; 
    unsafe {
        println!("-5的绝对值是{}",abs(-5));
         // 写入到标准输出
        write(1, data.as_ptr(), data.len());
    }
}
