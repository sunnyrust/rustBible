fn main() {
    let b_true =true;              //没有带上类型指示，Rust会在编译的时候做判断
    let b_false:bool =false; // 带有明确的类型注释
    println!("真={}  假={:#?}",b_true,b_false);

    if b_true{
        println!("真");
    }

    if !b_false{
        println!("假");
    }
}

