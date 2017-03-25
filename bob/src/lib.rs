pub fn is_all_cap(sent: &str) -> bool {
    sent.chars().all(|c| !c.is_lowercase())  // ignore non-slphabet
}

pub fn reply(sent: &str) -> &'static str {
    if sent.is_empty() {
        "Fine. Be that way!"
    } else if sent.ends_with("?") {
        "Sure."
    } else if is_all_cap(sent) {
        "Whoa, chill out!"
    } else{
        "Whatever."
    }
}
