use std::time::Instant;

fn main () {
    let start = Instant::now();
    println!("{}", prime_st(10_001));
    println!("Run time: {:?}", start.elapsed());
}
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }

    let mut i = 3;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}
fn prime_st(n: u32) -> u32 {
    let mut count = 0;
    let mut number = 1;

    while count < n {
        number += 1;

        if is_prime(number) {
            count += 1;
        }
    }
    
    number
}

