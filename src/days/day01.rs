day!(01, Some(71506), Some(209603), |part, input| -> u32 {
  let lines = input.lines().collect_vec();

  let values = lines
    .split(|line| line.trim().is_empty())
    .map(|group| group.iter().copied().map(parse_u32).sum::<u32>())
    .sorted_unstable()
    .rev()
    .take(3)
    .collect_vec();

  answer!(part, values[0], values.iter().sum::<u32>())
});
