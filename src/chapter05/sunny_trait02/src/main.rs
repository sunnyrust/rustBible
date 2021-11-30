use sunny_trait02::sunny::IntPrint;
/**
pub trait IntPrint {
    fn print(&self, formatter: String);

    fn add(&self, b: i32) -> i32;
}

impl IntPrint for i32{
    fn print(&self, formatter: String) {
        println!("{}, {}", self, formatter);
        
    }

    fn add(&self, b: i32) -> i32 {
        return self + b;
    }
}
*/
fn main() {
    3_i32.print("trait".to_string());
    println!("{}", 6_i32.add(9));
}
