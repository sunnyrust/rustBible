pub struct Circle {
    pub radius: f64,
}

pub trait Shape {
    fn area(&self) -> f64;
    // trait 可以提供默认的方法定义。
    fn show_area(&self) {
        println!("This shape has an area of {}", self.area());
    }
}

impl Circle {
    pub fn get_radius(&self) -> &f64 {
        &self.radius
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

fn main() {
    println!("Hello, world!");
}
