fn show(s: &str) {
    println!("{}", s);
}

fn main() {
    let mut sunny_string = String::from("Hello, world!");
    sunny_string += "Rustaceans";
    println!("{}", sunny_string);
    println!("size of &str: {}", std::mem::size_of::<&str>());
    println!("size of &[u8]: {}", std::mem::size_of::<&[u8]>());

    let s: &'static str = "hello";

    println!("s={}", s);

    // 将UTF-8序列转为字符串
    let tao = std::str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap();
    assert_eq!("道", tao);

    // 将16进制Unicode码位转为字符串
    assert_eq!("道", String::from("\u{9053}"));

    let unicode_x = 0x9053;
    let utf_x_hex = 0xe98193;
    let utf_x_bin = 0b111010011000000110010011;
    println!("unicode_x: {:b}", unicode_x);
    println!("utf_x_hex: {:b}", utf_x_hex);
    println!("utf_x_bin: 0x{:x}", utf_x_bin);

    let str_sunny = String::from("I'm a Rustaceans!");
    show(&str_sunny);
}
