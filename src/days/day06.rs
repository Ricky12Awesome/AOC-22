day!(Day06, Some(1623), Some(3774));
// day!(Day06);

impl Day06 {
  pub fn day(part: Part) -> Answer<usize> {
    let start = |window| window + Self::INPUT
      .as_bytes()
      .windows(window)
      .position(|bytes| bytes.iter().all_unique())
      .unwrap();

    let part1 = || start(4);
    let part2 = || start(14);

    answer!(part, part1, part2)
  }
}
