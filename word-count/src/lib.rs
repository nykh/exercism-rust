use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::collections::HashMap;

struct Counter<K: Hash + Eq + Clone> {
    map : HashMap<K, u32>,
}

impl<K: Hash + Eq + Clone> Counter<K> {
    fn new() -> Counter<K> {
        Counter { map: HashMap::new() }
    }

    fn from_iter<I: Iterator<Item = K>>(it: I) -> Counter<K> {
        let mut map = HashMap::<K, u32>::new();
        for item in it {
            *map.entry(item).or_insert(0) += 1;
        }
        Counter { map: map }
    }

    fn get_map(&self) -> HashMap<K, u32> { self.map.clone() }
} 

pub fn word_count(sent: &str) -> HashMap<String, u32> {
    let normalized = sent.replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), "")
                         .to_lowercase();
    let words = normalized.split_whitespace().map(|s| String::from(s));
    let counter = Counter::from_iter(words);
    counter.get_map()
}