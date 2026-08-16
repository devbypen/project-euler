use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", result());
    println!("Run time: {:?}", start.elapsed());
}

fn result() -> u32 {
    for i in 1..1000 {
        for j in i..(1000-i) {
            let k = 1000 - i - j;
            if i * i + j * j == k * k {
                return i * j * k;
            }
        }
    }
    0
}
