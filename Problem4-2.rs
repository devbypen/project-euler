use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut result = (0, 0, 0);

    for i in (100..1000).rev() {
        if i * i < result.0 {
            break;
        }
        for j in (100..=i).rev() {
            let check = i * j;
            if is_paridome(check) && check > result.0 {
                result = (check, i, j);
            }
        }
    }
    println!("{:?}", result);
    println!("Run time: {:?}", start.elapsed());
}

fn is_paridome(mut number: u32) -> bool {
    let original = number;
    let mut reversed = 0;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }
    reversed == original
}


