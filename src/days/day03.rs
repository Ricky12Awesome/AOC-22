use std::collections::BTreeSet;

day!(Day03, Some(8018), Some(2518));
// day!(Day03);

pub fn priority(c: char) -> u32 {
  match c {
    'a'..='z' => (c as u8 - b'a') as u32 + 1,
    'A'..='Z' => (c as u8 - b'A') as u32 + 27,
    _ => unreachable!(),
  }
}

fn item_type<const N: usize>(arr: [&str; N]) -> char {
  arr
    .into_iter()
    .map(str::chars)
    .map(BTreeSet::<_>::from_iter)
    .reduce(|a, b| a.intersection(&b).copied().collect())
    .and_then(|s| s.first().copied())
    .unwrap()
}

impl Day03 {
  pub fn day(part: Part) -> Answer<u32> {
    let part1 = || {
      Self::INPUT
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| [a, b])
        .map(item_type)
        .map(priority)
        .sum::<u32>()
    };

    let part2 = || {
      Self::INPUT
        .lines()
        .array_chunks::<3>()
        .map(item_type)
        .map(priority)
        .sum::<u32>()
    };

    answer!(part, part1, part2)
  }
}
