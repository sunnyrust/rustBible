// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_func<F: Fn()>(func: F) {
    func()
}

// 定义一个满足 `Fn` 限定的装包函数（wrapper function）。
fn foo() {
    println!("我是一个输入函数!");
}
fn main() {
    // 定义一个满足 `Fn` 限定的闭包。
    let closure = || println!("我是一个闭包。");
    call_func(closure);
    call_func(foo);

    let x = 6;
    let equal_to_x = |z| z == x;
    let y = 6;
    println!("判断两个数是不是相等：{}", equal_to_x(y));

    let array = [11, 12, 13, 14, 15, 16];
    println!("是否有能被3整除的数：{}", array.iter().any(|x| x % 3 == 0));

    let mut array = [4, 8, 1, 10, 0, 45, 12, 7];
    array.sort();
    println!("{:?}", array);

    use std::cmp::Ordering;
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    array.sort_by(desc);
    println!("{:?}", array);

    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    array.sort_by(desc);
    println!("{:?}", array);

    array.sort_by(|a, b| {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    println!("{:?}", array);
    array.sort_by(|a, b| b.cmp(a));
    println!("{:?}", array);

    array.sort_by(|a, b| (&-*a).cmp(&-*b));
    println!("{:?}", array);
}
