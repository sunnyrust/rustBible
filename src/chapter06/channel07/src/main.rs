use rayon::prelude::*;

const N: i32 = 200000;

fn main() {
    let mut primes = Vec::new();
    let mut cur = 10;
    // bootstrap
    for i in 2..cur {
        if primes.iter().all(|p| i % p != 0) {
            primes.push(i);
        }
    }
    while cur < N {
        let next = std::cmp::min(cur * cur, N);
        let mut addition: Vec<_> = (cur..next)
            .into_par_iter()
            .filter(|x| {
                for p in primes.iter() {
                    if x % p == 0 {
                        return false;
                    }
                    if x / p < *p {
                        break;
                    }
                }
                true
            })
            .collect();
        primes.append(&mut addition);
        cur = next;
    }
    for p in primes {
        println!("{}", p);
    }
}