use std::time::Instant;

fn main () {
    let start = Instant::now();
    println!("{}", prime_st(100_001));
    println!("Run time: {:?}", start.elapsed());
}

fn is_prime(n: u32, primes: &[u32]) -> bool {
    if n < 2 {
        return false;
    }

    for &p in primes {
        if p * p > n {
            break;
        }

        if n % p == 0 {
            return false;
        }
    }

    true
}

fn prime_st(n: usize) -> u32 {
    let mut primes = Vec::with_capacity(n);
    let mut number = 2;

    while primes.len() < n {
        if is_prime(number, &primes) {
            primes.push(number);
        }

        number += 1;
    }

    primes[n - 1]
}
