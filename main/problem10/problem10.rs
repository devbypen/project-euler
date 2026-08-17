use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", result(2_000_000));
    println!("Run time: {:?}", start.elapsed());
}

fn result(below_number: u64) -> u64 {
   let array = prime_numbres(below_number);

   let mut sum = 0;
   for i in array {
       sum += i;
   }

   sum
}

fn prime_numbres(below_number: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    for i in 2..=below_number {
        if is_prime(i) {
            vec.push(i);
        }
    }
    vec
}

fn is_prime(n: u64) -> bool {
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

