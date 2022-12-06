day!(Day06, Some(1623), Some(3774));
// day!(Day06);

impl Day06 {
  pub fn day(part: Part) -> Answer<usize> {
    let part1 = || {
      4 + Self::INPUT
        .as_bytes()
        .windows(4)
        .position(|bytes| bytes.iter().all_unique())
        .unwrap()
    };

    let part2 = || {
      14 + Self::INPUT
        .as_bytes()
        .windows(14)
        .position(|bytes| bytes.iter().all_unique())
        .unwrap()
    };

    answer!(part, part1, part2)
  }
}
