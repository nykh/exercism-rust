// A, E, I, O, U, L, N, R, S, T       1
// D, G                               2
// B, C, M, P                         3
// F, H, V, W, Y                      4
// K                                  5
// J, X                               8
// Q, Z                               10

use std::collections::HashMap;

pub fn get_scorer() -> HashMap<char, u32> {
    let mut char_scores = HashMap::new();
    for c in "AEIOULNRST".chars() {
        char_scores.insert(c, 1);
    }
    char_scores.insert('D', 2);
    char_scores.insert('G', 2);
    for c in "BCMP".chars() {
        char_scores.insert(c, 3);
    }
    for c in "FHVWY".chars() {
        char_scores.insert(c, 4);
    }
    char_scores.insert('K', 5);
    char_scores.insert('J', 8);
    char_scores.insert('X', 8);
    char_scores.insert('Q', 10);
    char_scores.insert('Z', 10);

    char_scores
}

pub fn score(xs: &str) -> u32 {
    let scorer = get_scorer();
    let default = 0;
    xs.to_uppercase()
      .chars()
      .map(|c| scorer.get(&c).unwrap_or(&default))
      .sum()
}

