use std::time::Instant;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

fn main() {
    let start = Instant::now();
    let mut result = 1;

    for i in 1..=20 {
        result = lcm(result, i);
    }

    println!("{}", result);
    println!("Run time: {:?}", start.elapsed());
}
