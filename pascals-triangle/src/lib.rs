pub struct PascalsTriangle {
    row_list: Vec<Vec<u32>>,
}

struct PascalsTriangleRowGenerator {
    row: Vec<u32>,
}

fn row_gen() -> PascalsTriangleRowGenerator {
    PascalsTriangleRowGenerator { row: vec![] }
}

impl Iterator for PascalsTriangleRowGenerator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.row.len() == 0 {
            self.row.push(1);
            return Some(self.row.clone())
        }

        self.row.push(0);
        for i in (1..self.row.len()).rev() {  // rev is important to not propgogate addition
            self.row[i] += self.row[i-1];
        }
        Some(self.row.clone())
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut row_list = Vec::new();
        let mut gen = row_gen();
        for _ in 0..row_count {
            row_list.push(gen.next().unwrap());
        }

        PascalsTriangle { row_list: row_list }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.row_list.clone()
    }

}
