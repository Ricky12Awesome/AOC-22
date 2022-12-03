day!(Day03, Some(8018), Some(2518));
// day!(Day03);

pub fn priority(c: char) -> u32 {
  match c {
    'a'..='z' => (c as u8 - b'a') as u32 + 1,
    'A'..='Z' => (c as u8 - b'A') as u32 + 27,
    _ => unreachable!(),
  }
}

fn item_type((a, b): (&str, &str)) -> char {
  for a in a.chars() {
    for b in b.chars() {
      if a == b {
        return a;
      }
    }
  }

  unreachable!()
}

fn item_type_2([a, b, c]: [&str; 3]) -> char {
  for a in a.chars() {
    for b in b.chars() {
      for c in c.chars() {
        if a == b && b == c {
          return a;
        }
      }
    }
  }

  unreachable!()
}

impl Day03 {
  pub fn day(part: Part) -> Answer<u32> {
    let part1 = || Self::INPUT
      .lines()
      .map(|line| line.split_at(line.len() / 2))
      .map(item_type)
      .map(priority)
      .sum::<u32>();

    let part2 = || Self::INPUT
      .lines()
      .array_chunks::<3>()
      .map(item_type_2)
      .map(priority)
      .sum::<u32>();

    answer!(part, part1, part2)
  }
}
