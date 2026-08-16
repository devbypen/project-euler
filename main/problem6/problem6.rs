use std::time::Instant;

fn main () {
    let start = Instant::now();
    println!("{}", diff(100));
    println!("Run time: {:?}", start.elapsed());
}
fn diff(number: u32) -> u32 {
    let sum_o_s: u32 = sum_of_square(&number);
    let square_o_s: u32 = square_of_sum(&number);

    square_o_s - sum_o_s
}

fn sum_of_square(n: &u32) -> u32 {
    let mut result = 0;

    for i in 1..=*n {
        result += i * i;
    }
    result
}

fn square_of_sum(n: &u32) -> u32 {
    let mut result = 0;
    for i in 1..=*n {
        result += i;
    }
    
    result * result
}
