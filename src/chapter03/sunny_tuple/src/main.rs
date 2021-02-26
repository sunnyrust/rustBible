fn tup_sunny(){
    let tup_sunny: (&str, i32, &str)=("Sunny",46,"Rustaceans");
    let (name, age, job) = tup_sunny;
    println!("{},{} years old, he's a {}",name, age, job);
}
fn lantern_festival(){
    let tup = ("🍡🍡🍡","2021-02-26", "元宵节","Sunny","节日快乐！","🎉🎉🎉");
    println!("{} {},今天是{},{}祝所有的Rustaceans{} {}",tup.0, tup.1, tup.2, tup.3, tup.4, tup.5);
}
fn main() {
    tup_sunny();
    lantern_festival();
}
