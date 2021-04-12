pub fn speak(){
    println!("叫叫");
}

fn private_cat_speak(){
    println!("喵喵");
}

pub fn indirect_access(){
    private_cat_speak();
}