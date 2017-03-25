pub fn square_of_sum(n: u64) -> u64 {
    let sum: u64 = (1u64..(n+1)).sum();
    sum * sum
}

pub fn sum_of_squares(n: u64) -> u64 {
    (1u64..(n+1)).fold(0u64, |acc, x| acc + x * x)
}

pub fn difference(n: u64) -> u64 {
    let x = square_of_sum(n) as i64;
    let y = sum_of_squares(n) as i64;
    (x - y).abs() as u64
}
