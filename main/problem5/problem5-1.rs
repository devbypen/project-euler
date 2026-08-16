use std::collections::HashMap;
use std::time::Instant;   

fn main() {
    let start = Instant::now();
    let mut dict: HashMap <u32, Vec<u32>> = HashMap::new();

    for i in 1..=20 {
        dict.insert(i, prime_factiorization(i));
    }
    let result = smallest_numb(&dict);

    println!("{}", result);
    println!("Run time: {:?}", start.elapsed());
}

fn prime_factiorization(mut number: u32) -> Vec<u32> {
    let mut factors = Vec::new();

    let mut divisor = 2;

    while divisor * divisor <= number {
        while number % divisor == 0 {
            factors.push(divisor);
            number /= divisor;
        }

        divisor += 1;
    }

    if number > 1 {
        factors.push(number);
    }

    factors
}

fn smallest_numb(dict: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut prime_powers: HashMap<u32, u32> = HashMap::new();

    for (_, value) in dict {
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for &prime in value {
            *counts.entry(prime).or_insert(0) += 1;
        }

        for (prime, exponent) in counts {
            let current = prime_powers.entry(prime).or_insert(0);
            if exponent > *current {
                *current = exponent;
            } 
        }
    }

    let mut result = 1;

    for (prime, exponent) in prime_powers {
        result *= prime.pow(exponent);
    }

    result
}

