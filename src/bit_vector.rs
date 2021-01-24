trait BitVector {
  fn get_bit(&self, pos: usize) -> Option<bool>;

  fn to_bit_string(&self) -> std::string::String;
}

impl BitVector for Vec<u8> {
  fn get_bit(&self, pos: usize) -> Option<bool> {
    let u32_size = std::mem::size_of::<u32>();
    let byte_position = pos / (u32_size * 8);
    let checker = 1 << (pos % (u32_size * 8));
    if byte_position >= self.len() {
      None
    } else {
      Some((self[byte_position] & checker) == checker)
    }
  }

  fn to_bit_string(&self) -> std::string::String {
    self.iter().rev()
      .map(|it| {
        let mut byte = 0x80;
        let mut byte_str = Vec::new();
        while byte > 0x00 {
          if (it & byte) == byte {
            byte_str.push('1');
          } else {
            byte_str.push('0');
          }
          byte >>= 1;
        }
        byte_str
      })
      .flatten()
      .collect()
  }
}

fn from_num(num: u32) -> Vec<u8> {
  let u32_size = std::mem::size_of::<u32>();
  let mut bits = vec![0; u32_size];

  let mut rev_it = bits.len() + 1;
  let mut char_bits: u8 = 0x80;
  let mut checker: u32 = 0x80000000;
  while rev_it >= 1 && checker > 0x00000000 {
    if char_bits == 0x00 {
      rev_it -= 1;
      char_bits = 0x80;
    }

    if (num & checker) == checker {
      bits[rev_it - 2] |= char_bits;
    }

    checker = checker >> 1;
    char_bits = char_bits >> 1
  }
  bits
}

#[cfg(test)]
mod bit_vector {
  use super::*;

  #[test]
  fn get_bit_successfully() {
    let four = from_num(4);
    println!("{}", four.to_bit_string());
    assert_eq!(four.get_bit(0), Some(false));
    assert_eq!(four.get_bit(2), Some(true));

    let thirty_two = from_num(32);
    assert_eq!(thirty_two.get_bit(0), Some(false));
    assert_eq!(thirty_two.get_bit(5), Some(true));

    let sixty_five = from_num(65);
    assert_eq!(sixty_five.get_bit(0), Some(true));
    assert_eq!(sixty_five.get_bit(6), Some(true));
    assert_eq!(sixty_five.get_bit(2), Some(false));
  }

  #[test]
  fn to_bit_string_successfully() {
    let four = from_num(4);
    assert_eq!(four.to_bit_string(), "00000000000000000000000000000100");

    let thirty_two = from_num(32);
    assert_eq!(thirty_two.to_bit_string(), "00000000000000000000000000100000");

    let sixty_five = from_num(65);
    assert_eq!(sixty_five.to_bit_string(), "00000000000000000000000001000001");
  }
}
