pub trait Cipher {
    fn encode_char(&mut self, c: char) -> char;
    fn decode_char(&mut self, c: char) -> char;

    fn encode(&mut self, cs: &String) -> String {
        cs.chars()
          .map(|c| {
              if c.is_alphabetic() {
                  self.encode_char(c)
              } else {
                  c
              }
          }).collect::<String>()
    }

    fn decode(&mut self, cs: &String) -> String {
        cs.chars()
          .map(|c| {
              if c.is_alphabetic() {
                  self.decode_char(c)
              } else {
                  c
              }
          }).collect::<String>()
      }
}
