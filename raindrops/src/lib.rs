pub fn raindrops(x: i64) ->  String {
    let mut res = String::with_capacity(5 * 3);  // maximum of PlingPlangPlong
    if x % 3 == 0 {
        res.push_str("Pling")
    }

    if x % 5 == 0 {
        res.push_str("Plang")
    }

    if x % 7 == 0 {
        res.push_str("Plong")
    }

    if res.is_empty() {
        x.to_string()
    } else {
        res
    }
}
