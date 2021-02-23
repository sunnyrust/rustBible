//!  注释程序例子
//!  这个是一个解释注释的例子
//! # Example：
//! ```
//!  let msg =common("Rust 注释");
//! ```
//! - 目录
//! - [`注释`]
//!
//! [`注释`]: https://github.com/sunnyrust/rustBible/blob/master/books/05.md

/// 注释函数
/// # Example:
///
/// ```
/// fn common(s_msg:&str) -> String {
///     let mut  result = "Hello ".to_string();
///     result=result.to_owned() + &s_msg.to_string();
///     return result ;
///  }
/// ```
fn common(s_msg:&str) -> String {
    let mut  result = "Hello ".to_string();
    result=result.to_owned() + &s_msg.to_string();

    return result ;
}
/// 主函数 
/// # 调用
/// ## main()
///         每个项目只能有一个主函数。
fn main() {
    let msg =common("Rust 注释");
    println!("{}",msg);
}