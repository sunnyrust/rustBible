fn main() {
    let i_num =100;
    println!("{}转换成char---{}",i_num,i_num as u8 as char);
    println!("{}----{}", "B","B".len()); // .len() gives the size in bytes
    println!("{}----{}", "ß","ß".len());
    println!("{}---{}", "锈","锈".len());
    println!("{}---{}", "😺","😺".len());
    println!("{}----{}", "⚠️","⚠️".len());
    let ico = "😂";
    println!("The ico is actually {}----{}", ico,ico.len());
}
