macro_rules! to_placeholder {
    ($x: expr) => {
        "{}"
    }
}

macro_rules! sunny_print {
    ($first: expr $(, $x: expr)* $(,)?) => {
        println!(concat!("{}", $(" ", to_placeholder!($x)), *), $first, $($x), *)
    }
}


fn main() {
    // Rust 每条语句都要使用；作为结尾
    // println! 这种以!结尾的是宏语句调用，在Rust里面有大量的宏语句
    println!("换行输出!");  // 换行输出内容

    print!("不换行");            // 不换行输出

    print!("换行输出\n");  // 换行输出

    println!("测试println的{}。","功能");   // 传递参数的方式

    sunny_print!(1);
    sunny_print!(1, 2);
    sunny_print!(1, "test");
    sunny_print!(1, "test", 3);
}