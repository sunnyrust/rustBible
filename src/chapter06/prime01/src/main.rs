fn main() {
    for i in 1..100 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}