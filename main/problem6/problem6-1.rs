use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", diff(100));
    println!{"Run time: {:?}", start.elapsed()};
}
fn diff(n: u32) -> u32 {
    let sum = n * (n+1) / 2;
    let sum_of_squares = n * (n+1) * (2 * n + 1) / 6;

    sum * sum - sum_of_squares
}
