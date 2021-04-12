fn main() {
    let sunny_str = String::from("I am a Rustaceans!🍇");

    let len = calculate_length(&sunny_str);

    println!("'{}' 的长度是{}。", sunny_str, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}