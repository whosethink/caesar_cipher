#[derive(Debug)]
pub struct Cipher {
  shift: i8
}

impl Cipher {
  pub fn new(shift: i8) -> Self {
    Cipher {
      shift
    }
  }

  pub fn cipher_str(&self, s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
      result.push(self.cipher_char(c));
    }
    return result
  }

  fn cipher_char(&self, c: char) -> char {
    return if c.is_ascii() { self.cipher_ascii_char(c) } else { c };
  }

  fn cipher_ascii_char(&self, c: char) -> char {
    let mut result = c as u8;
    match result {
      65..=90 | 97..=122 => {
        let flag = result <= 90;
        result = if self.shift > 0 { result + (self.shift.abs() as u8)} else { result - (self.shift.abs() as u8)};
        if (flag && result < 65) || (!flag && result < 97) {
          result = 26 + result;
        } else if (flag && result > 90) || (!flag && result > 122) {
          result = result - 26;
        }
      },
      _ => {}
    }
    return result as char;
  }
}