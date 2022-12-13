day!(03, Some(8018), Some(2518), |part, input| -> u32 {
  answer!(
    part,
    input
      .lines()
      .map(|line| line.split_at(line.len() / 2))
      .map(|(a, b)| [a, b])
      .map(item_type)
      .map(priority)
      .sum::<u32>(),
    input
      .lines()
      .array_chunks::<3>()
      .map(item_type)
      .map(priority)
      .sum::<u32>()
  )
});

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
