#[derive(Debug)]
#[allow(dead_code)]
enum Fruit {
    Apple,
    WaterMelon,
    Grape,
    Peach,
}
fn main() {
    let fruit = Fruit::Apple;
    match fruit {
        Fruit::Apple => println!("Apple:  {}","  🍎   "),
        Fruit::WaterMelon | Fruit::Grape => {
            println!("🍉🍉🍉 or 🍇🍇🍇");
        },
        _ => println!("🍑🍑🍑"),
    };

    println!("🍉🍉🍉 or 🍇🍇🍇");
}
