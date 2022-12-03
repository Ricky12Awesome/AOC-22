day!(Day01, Some(71506), Some(209603));
// day!(Day01);

impl Day01 {
  pub fn day(part: Part) -> Answer<u32> {
    let lines = Self::INPUT.lines().collect_vec();
    let values = lines
      .split(|line| line.trim().is_empty())
      .map(|group| group.iter().map(|n| n.parse::<u32>().unwrap()).sum::<u32>())
      .sorted_unstable()
      .rev()
      .take(3)
      .collect_vec();

    let part1 = || values[0];
    let part2 = || values.iter().sum::<u32>();

    answer!(part, part1, part2)
  }
}
