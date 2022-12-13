day!(06, Some(1623), Some(3774), |part, input| -> usize {
  let start = |window| {
    input
      .as_bytes()
      .windows(window)
      .position(|bytes| bytes.iter().all_unique())
      .unwrap()
      + window
  };

  answer!(part, start(4), start(14))
});
