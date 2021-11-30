use std::fmt;
//下面的impl ... for Point相当于derive语句。
//#[derive(Debug)]
pub struct SunnyPoint<T> {
    x: T,
    y: T,
}
impl<T>   SunnyPoint<T> {
    fn fmt<T:fmt::Debug + Clone>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SunnyPoint {{ x:{} , y:{} }}", &self.x, &self.y)
    }
}
fn main() {
    let origin = SunnyPoint { x: 0, y: 0 };

    println!("The origin is: {}", origin);
}
