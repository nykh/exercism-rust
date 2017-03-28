pub fn has_invalid_char(xs: &str) -> bool {
    xs.chars().any(|c| !c.is_digit(10) && c != ' ')
}

pub fn is_valid(xs: &str) -> bool {
    if has_invalid_char(xs) {
        false
    } else {
        let mut ds: Vec<u32> = xs.chars()
                       .filter(|c| c.is_digit(10))
                       .map(|c| c.to_digit(10).unwrap())
                       .collect();
        if ds.len() == 1 {
            return false
        }

        let mut i: usize = ds.len() - 2;  // len > 1 => len - 2 >= 0
        loop {
            ds[i] = match ds[i] {
                0...4 => 2 * ds[i],
                5...8 => (2 * ds[i]) % 10 + 1,
                9     => 9,
                _     => panic!(),
            };

            i = match i {
                0...1 => break,
                _ => i - 2
            };
        }
        let sum = ds.iter().fold(0, |acc, d| acc + d);
        sum % 10 == 0
    }
}