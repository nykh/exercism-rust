use std::iter::FromIterator;
use std::ascii::AsciiExt;

fn codec_char(c: char) -> char {
    let id = (c as u8) - ('a' as u8);  // 0 .. 25 
    ((25 - id) + 'a' as u8) as char
}

pub fn encode(text: &str) -> String {
    let mut v: Vec<char> = Vec::new();
    let lc = text.to_lowercase();
    let it = lc.chars().filter(|c| c.is_alphanumeric() && c.is_ascii());
    for (i, c) in it.enumerate() {
        if i % 5 == 0 && i > 0 {
            v.push(' ');
        }

        v.push(match c {
            'a'...'z'=> codec_char(c),
            '0'...'9'=> c,
            _ => panic!(),
        });
    }
    String::from_iter(v)  
}

pub fn decode(text: &str) -> String {
    String::from_iter(text.chars()
       .filter(|c| c.is_alphanumeric())
       .map(|c| {
           if c.is_alphabetic() {
               codec_char(c)
           } else {
               c
           }
       }))
}