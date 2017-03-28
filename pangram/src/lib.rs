type IdT = usize;

fn from_char(c: char) -> Option<IdT> {
    let num = c as u32;
    const ASCII_THRESH: u32 = 128;
    if num < ASCII_THRESH && c.is_alphabetic() {
        Some(((c as u8) - ('a' as u8)) as IdT)
    } else {
        None
    }
}

pub fn is_pangram(sent: &'static str) -> bool {
    let mut alphabet = vec![false; 26];
    let mut unfilled = 26;
    for c in sent.to_lowercase().chars() {
        if let Some(id) = from_char(c) {
            if !alphabet[id] {
                unfilled -= 1;
                alphabet[id] = true;
                if unfilled == 0 {
                    break
                }
            }
        }
    }
    
    alphabet.iter().all(|x| *x)
}