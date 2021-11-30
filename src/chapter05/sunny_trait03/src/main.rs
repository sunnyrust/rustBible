use std::fmt;
//下面的impl ... for Point相当于derive语句。
//#[derive(Debug)]
pub struct SunnyPoint {
    x: i32,
    y: i32,
}
 
impl fmt::Debug for SunnyPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SunnyPoint")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}
 

fn main() {
    let origin = SunnyPoint { x: 0, y: 0 };

    println!("The origin is: {:?}", origin);
}
