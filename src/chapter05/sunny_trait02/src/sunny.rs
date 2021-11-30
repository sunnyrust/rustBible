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
