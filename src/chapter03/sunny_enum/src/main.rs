#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),// 这个枚举成员是四个u8类型的元祖
    V6(String), // 这个枚举成员是String类型
}

fn main() {
    let ipv4 = IpAddr::V4(127, 0, 0, 1);
    let ipv6 = IpAddr::V6(String::from("::1"));
    println!("{:?}\n{:?}",ipv4,ipv6);//V4(127, 0, 0, 1) 
                                     //V6("::1")
}

