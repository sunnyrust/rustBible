fn main() {
    let f32:f32=0.32;   // float32 要做类型指定
    let f64 =0.64; // 默认是float64

    println!("f32={}  f64={:#?}",f32,f64);

    println!("The smallest f32 is {} and the biggest f32 is {}.", std::f32::MIN, std::f32::MAX); 
    println!("The smallest f64 is {} and the biggest f64 is {}.", std::f64::MIN, std::f64::MAX);
}
