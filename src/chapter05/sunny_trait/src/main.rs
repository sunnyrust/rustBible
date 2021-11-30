#[allow(dead_code)]
struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

trait Shape {
    fn area(&self) -> f64;
    // trait 可以提供默认的方法定义。
    fn show_area(&self) {
        println!("This shape has an area of {}", self.area());
    }
}

impl Circle {
    #[allow(dead_code)]
   fn get_radius(&self) -> &f64 {
        &self.radius
    }

    fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl  Shape for Circle {
    fn area(&self) -> f64{
        std::f64::consts::PI*self.radius*self.radius
    }
}


fn main() {
    let  mut  c=Circle{
        x:0.0f64,
        y:0.0f64,
        radius:3f64
    };
    // c.set_radius(5f64);
    let area=c.area();
    println!("面积= {}", area);
    c.set_radius(9.1f64);
    c.show_area();
}
