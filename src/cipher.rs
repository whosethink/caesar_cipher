#[derive(Debug)]
pub struct Cipher {
  shift: i8
}

impl Cipher {
  pub fn new(shift: i8) -> Self {
    assert!(shift <= 25 && shift >= -25, "CaesarCipher: shift => {}", shift);
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

#[cfg(test)]
mod cipher_test {

  use super::*;

  #[test]
  fn cipher_ascii_char_test() {
    let cipher = Cipher::new(0);
    assert_eq!(cipher.cipher_ascii_char('A'), 'A');
    assert_eq!(cipher.cipher_ascii_char('a'), 'a');
    assert_eq!(cipher.cipher_ascii_char('Z'), 'Z');
    assert_eq!(cipher.cipher_ascii_char('z'), 'z');

    let cipher = Cipher::new(1);
    assert_eq!(cipher.cipher_ascii_char('A'), 'B');
    assert_eq!(cipher.cipher_ascii_char('a'), 'b');
    assert_eq!(cipher.cipher_ascii_char('Z'), 'A');
    assert_eq!(cipher.cipher_ascii_char('z'), 'a');

    let cipher = Cipher::new(25);
    assert_eq!(cipher.cipher_ascii_char('A'), 'Z');
    assert_eq!(cipher.cipher_ascii_char('a'), 'z');
    assert_eq!(cipher.cipher_ascii_char('Z'), 'Y');
    assert_eq!(cipher.cipher_ascii_char('z'), 'y');

    let cipher = Cipher::new(-1);
    assert_eq!(cipher.cipher_ascii_char('A'), 'Z');
    assert_eq!(cipher.cipher_ascii_char('a'), 'z');
    assert_eq!(cipher.cipher_ascii_char('Z'), 'Y');
    assert_eq!(cipher.cipher_ascii_char('z'), 'y');

    let cipher = Cipher::new(-25);
    assert_eq!(cipher.cipher_ascii_char('A'), 'B');
    assert_eq!(cipher.cipher_ascii_char('a'), 'b');
    assert_eq!(cipher.cipher_ascii_char('Z'), 'A');
    assert_eq!(cipher.cipher_ascii_char('z'), 'a');
  }

  #[test]
  fn cipher_char_test() {
    let cipher = Cipher::new(1);
    assert_eq!(cipher.cipher_char('A'), 'B');
    assert_eq!(cipher.cipher_char('@'), '@');
    assert_eq!(cipher.cipher_char(' '), ' ');
    assert_eq!(cipher.cipher_char('咖'), '咖');
  }
}