//!
//! 在Cargo.toml里面添加
//! ```
//!  [dependencies]
//! ferris-says = "0.2"
//! ```
//! 在main.rs 头上引用
//! ```
//! use ferris_says::say; // from the previous step
//! use std::io::{stdout, BufWriter};
//! ```
//! 

use ferris_says::say; // from the previous step
use std::io;
use std::io::{stdout, BufWriter};

/// 用于使用ferris_says显示内容
fn says(msg:&str){
    let stdout = stdout();
    let message = String::from(msg);
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

///   ____________________
/// ```plain
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
fn main() {
    says("I'm a  Rustaceans!");
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let msg=format!("You guessed: {}", guess);
    says(msg.as_str());
}
