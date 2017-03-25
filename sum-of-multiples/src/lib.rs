use std::collections::HashSet;

struct Multiple {
    root: u64,
    curr: u64,
}

impl Iterator for Multiple {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.curr += self.root;
        Some(self.curr)
    }
}

fn multiples(root: u64) -> Multiple {
    Multiple { root: root, curr: 0u64 }
}


pub fn sum_of_multiples(thresh: u64, xs: &Vec<u64>) -> u64 {
    // for each x in xs, create list of multiples that are less than thresh
    let set: HashSet<u64> = HashSet::new();

    xs.iter()
      .map(|x| multiples(*x).take_while(|m| *m < thresh))
      .fold(set, |mut acc, xs| { acc.extend(xs); acc } )
      .iter()
      .sum()
}