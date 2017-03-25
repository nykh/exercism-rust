pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    // 1 + 2 + 4 + ... 2^63 = 0b1111...111 = (-1) when signed
    -1i64 as u64
}
