fn main() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    println!(
        "v1={}  v2={}   v3={}   v4={}",
        add_one_v1(5),
        add_one_v2(5),
        add_one_v3(5),
        add_one_v4(5)
    );
    let i=5;
    let add_one_v5 = |x| -> i32 {x+1 }(i);
    println!("v5= {}", add_one_v5);

    let foo = || true;
    println!("没有参数的闭包: {}", foo());

    let mut count = 5;
    let mut foo = || {
        count += 1;
        println!("v6= {}", count);
    };

    // Call the closure.
    foo();
    foo();
}
