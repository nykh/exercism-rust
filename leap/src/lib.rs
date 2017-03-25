pub fn is_leap_year(yr: u64) -> bool {
    if yr % 4 == 0 {
        if yr % 100 == 0 {
            yr % 400 == 0
        } else {
            true
        }
    } else {
        false
    }    
}
