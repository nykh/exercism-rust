extern crate chrono;
extern crate time;
use std::ops::*;
use chrono::*;
use time::Duration;

pub fn after(d: DateTime<UTC>) -> DateTime<UTC> {
    let giga: Duration = Duration::seconds(10i64.pow(9));
    d.add(giga)
}
