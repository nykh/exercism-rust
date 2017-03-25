pub fn hamming_distance(xs: &str, ys: &str) -> Result<i32, &'static str> {
    if xs.len() != ys.len() {
        Err("x and y are not of the same length")
    } else {
        let sum = if xs.len() > 0 {
            xs.chars()
              .zip(ys.chars())
              .map(|(x, y)| (x != y) as i32)
              .sum()
        } else {
            0
        };
        
        Ok(sum)
    }
}