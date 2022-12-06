
pub fn parse_u32(str: &str) -> u32 {
  str.parse().unwrap()
}

pub fn parse_usize(str: &str) -> usize {
  str.parse().unwrap()
}

pub fn char_to_u32(c: char) -> u32 {
  c.to_digit(10).unwrap()
}

pub fn char_to_usize(c: char) -> usize {
  c.to_digit(10).unwrap() as usize
}
