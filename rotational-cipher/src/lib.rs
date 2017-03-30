mod cipher;
use cipher::Cipher;

struct RotationCipher {
    n: u8,
}

impl RotationCipher {
    fn new(n: u8) -> RotationCipher {
        RotationCipher {n: n % 26}
    }

    #[inline]
    fn get_base(&self, x: char) -> u8 {
        match x {
            'a'...'z' => 'a' as u8,
            'A'...'Z' => 'A' as u8,
            _ => panic!()
        }
    }
}

impl Cipher for RotationCipher {
    fn encode_char(&mut self, x: char) -> char {
        let base = self.get_base(x);
        let id = (x as u8) - base;
        let newid = (id + self.n) % 26;
        (newid + base) as char
    }

    fn decode_char(&mut self, x: char) -> char {
        let base = self.get_base(x);
        let id = (x as u8) - base;
        let newid = (id + (26 - self.n)) % 26;
        (newid + base) as char
    }
}

pub fn rotate(xs: &str, n: usize) -> String {
    let mut rot = RotationCipher::new(n as u8);
    rot.encode(&String::from(xs))
}
